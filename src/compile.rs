use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use kismesis::{
    html::{self, ScopedError},
    parser::errors::Err,
    plugins,
    reporting::{DrawingInfo, Report, ReportKind},
    KisTokenId, Kismesis, KismesisError,
};

/// An Enum containing all the possible errors that the process of compiling a project might emit
pub enum Error {
    IO(io::Error, PathBuf),
    NoMainTemplate,
    OutputNotInOutputFolder(PathBuf),
    Parse(Vec<Err>, KisTokenId),
    TriedToGetNonExistentTemplate(KisTokenId),
}

/// Loads all the plugins in the plugins directory
#[cfg(feature = "plugins")]
fn check_for_plugins(program_path: &directories::ProjectDirs, engine: &mut Kismesis) {
    let plugin_dir = program_path.data_dir().join("plugins");
    let Ok(plugin_paths) = fs::read_dir(plugin_dir) else { return };
    for entry in plugin_paths {
        let entry = entry.unwrap();
        let path = entry.path();
        let data = path.join("plugin.ron");
        let data =
            ron::from_str::<plugins::PluginData>(&fs::read_to_string(data).unwrap()).unwrap();
        let plugin_path = path.join("plugin.wasm");
        engine.register_plugin(data.name, &plugin_path);
    }
}

/// Loads all the plugins in the plugins directory
#[cfg(not(feature = "plugins"))]
fn check_for_plugins(program_path: &directories::ProjectDirs, engine: &mut Kismesis) {
    println!("Plugins are not being registered because this version of Kismesis was compiled without plugins")
}

/// Compile a kismesis project
pub(crate) fn compile_project() {
    let mut errors = Vec::new();
    let mut engine = Kismesis::new();
    let program_path =
        directories::ProjectDirs::from("net.ampersandia", "ampersandia", "kismesis").unwrap();

    check_for_plugins(&program_path, &mut engine);


    let main_template_path = PathBuf::from("templates/main.ks");
    let template_paths = recursive_crawl(&PathBuf::from("templates")).0;

    for path in template_paths {
        match engine.register_file(path) {
            Ok(x) => {
                engine.register_template(x);
            }
            Err(x) => errors.push(x.into()),
        }
    }

    if !errors.is_empty() {
        report_errors(&errors, &engine);
        return;
    }

    let Some(main_template_id) = engine.verify_template_id(main_template_path) else {
        errors.push(Error::NoMainTemplate);
        report_errors(&errors, &engine);
        return;
    };
    let input_paths = recursive_crawl(&PathBuf::from("input")).0;

    for path in input_paths {
        let parsed_file = match engine.register_file(path) {
            Ok(mut x) => {
                x.template = Some(main_template_id.clone());
                x
            }
            Err(x) => {
                errors.push(x.into());
                continue;
            }
        };
        match html::compile(&parsed_file, &engine) {
            Ok(x) => {
                let output_path = PathBuf::from("output");
                let file = match engine.get_file(parsed_file.file_id) {
                    Some(x) => x,
                    None => {
                        errors.push(Error::TriedToGetNonExistentTemplate(parsed_file.file_id));
                        return;
                    }
                };
                if let Some(path) = &file.path {
                    let mut output_path =
                        output_path.join::<PathBuf>(path.iter().skip(1).collect());
                    output_path.set_extension("html");
                    match output_path.parent() {
                        Some(parent) => match std::fs::create_dir_all(parent) {
                            Ok(_) => (),
                            Err(x) => {
                                errors.push(Error::IO(x, output_path.clone()));
                                continue;
                            }
                        },
                        None => errors.push(Error::OutputNotInOutputFolder(output_path.clone())),
                    };
                    let mut file = match File::create(&output_path) {
                        Ok(x) => x,
                        Err(x) => {
                            errors.push(Error::IO(x, output_path.clone()));
                            continue;
                        }
                    };
                    let file_text = x.to_string();
                    match write!(file, "{}", file_text) {
                        Ok(x) => {
                            engine.drop_id(&parsed_file.file_id);
                            x
                        }
                        Err(x) => {
                            errors.push(Error::IO(x, output_path.clone()));
                            continue;
                        }
                    };
                }
            }
            Err(errors) => {
                for error in errors {
                    error.report(ReportKind::Error, &DrawingInfo::default(), &engine, 0);
                }
            }
        }
    }

    if !errors.is_empty() {
        report_errors(&errors, &engine)
    }
}

/// Recursively get all Kismesis files
pub fn recursive_crawl(path: &Path) -> (Vec<PathBuf>, Vec<io::Error>) {
    let mut errors = Vec::new();
    let mut paths = Vec::new();
    let entries = match fs::read_dir(path) {
        Ok(x) => x,
        Err(x) => return (vec![], vec![x]),
    };
    for entry in entries {
        let entry = match entry {
            Ok(x) => x,
            Err(x) => {
                errors.push(x);
                continue;
            }
        };
        let path = entry.path();
        if path.is_dir() {
            let (mut a, mut b) = recursive_crawl(&path);
            errors.append(&mut b);
            paths.append(&mut a)
        } else if let Some(ext) = path.extension() {
            if ext.to_string_lossy() == "ks" {
                paths.push(path)
            }
        }
    }

    (paths, errors)
}

pub fn report_errors(errors: &[Error], engine: &Kismesis) {
    eprintln!("\n");
    for error in errors {
        match error {
            Error::IO(error, path) => eprintln!("Error reading `{}`: {}", path.to_string_lossy(), error),
            Error::NoMainTemplate => eprintln!("Coudln't compile project because it doesn't have a template in templates/main.ks"),
            Error::OutputNotInOutputFolder(path) => eprintln!("Tried to output {} to a location outside the project's output folder.\n\nThis is meant to be impossible, please contact the developer at https://ampersandia.net/", path.to_string_lossy()),
            Error::Parse(error, id) => for error in error {
                let error = Into::<ScopedError<_>>::into((*id, error.clone().unpack()));
				error.report(ReportKind::Error, &DrawingInfo::default(), engine, 0);
			},
			Error::TriedToGetNonExistentTemplate(id) => eprintln!("Tried to get a non-existent kismesis template {:?}", id)
        }
    }
}

impl From<KismesisError> for Error {
    fn from(value: KismesisError) -> Self {
        match value {
            KismesisError::IOError(x, y) => Error::IO(x, y),
            KismesisError::ParseError(x, y) => Error::Parse(x, y),
        }
    }
}
