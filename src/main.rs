mod compile;
#[cfg(feature = "server")]
mod server;

use clap::{Parser, Subcommand};
use compile::report_errors;
use kismesis::options::Settings;
use once_cell::sync::Lazy;
use ron::{de::SpannedError, ser::PrettyConfig};
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(name = "kismesis")]
struct Kismesis {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Build,
    New { name: Option<String> },
    Run,
}

static DEFAULT_PRETTY_SETTINGS: Lazy<PrettyConfig> =
    Lazy::new(|| PrettyConfig::new().indentor("\t".to_owned()));

const DEFAULT_TEMPLATE: &str = r#"$mut title

<!doctype html>
<html lang="EN-US":
	<head:
		<title | @title>
	>
	<body:
		<header:
			<span | My Website>
		>
		<main:
			<content!>
		>
	>
>
"#;

const DEFAULT_INDEX: &str = r#"$const title = "Example title"

# @title

This is an example page

## And this is a subheading
With its example content
"#;

#[cfg(feature = "server")]
#[actix_web::main]
async fn main() {
    let cli = Kismesis::parse();

    match cli.command {
        Some(Commands::Build) => {
            let settings = match get_settings() {
                Ok(x) => x,
                Err(CouldntParseSettings::CouldntRead) => {
                    return eprintln!("Couldn't read .kismet file")
                }
                Err(CouldntParseSettings::CouldntParse(error)) => {
                    return eprintln!("Couldn't parse .kismet file: {error}")
                }
            };
            let mut engine = kismesis::Kismesis::new();
            engine.settings = settings;
            match compile::compile_project(&mut engine) {
                Ok(()) => (),
                Err(errors) => report_errors(&errors, &engine),
            }
        }
        Some(Commands::New { name }) => new(name.unwrap_or(".".to_string())),
        Some(Commands::Run) => server::start().await.unwrap(),
        None => println!("The Kismesis TUI is currently not implemented"),
    }
}

#[cfg(not(feature = "server"))]
fn main() {
    let cli = Kismesis::parse();

    match cli.command {
        Some(Commands::Build) => {
            let settings = match get_settings() {
                Ok(x) => x,
                Err(CouldntParseSettings::CouldntRead) => {
                    return eprintln!("Couldn't read .kismet file")
                }
                Err(CouldntParseSettings::CouldntParse(error)) => {
                    return eprintln!("Couldn't parse .kismet file: {error}")
                }
            };
            let mut engine = kismesis::Kismesis::new();
            engine.settings = settings;
            match compile::compile_project(&mut engine) {
                Ok(()) => (),
                Err(errors) => report_errors(&errors, &engine),
            }
        }
        Some(Commands::New { name }) => new(name.unwrap_or(".".to_string())),
        Some(Commands::Run) => {
            eprintln!("This version of Kismesis was not compiled with the server feature.")
        }
        None => println!("The Kismesis TUI is currently not implemented"),
    }
}

fn new(name: String) {
    let name = PathBuf::from(name);
    match name.try_exists() {
        Ok(x) if x => match fs::read_dir(&name) {
            Ok(x) => {
                if x.count() != 0 {
                    return eprintln!("The given path exists and is not empty, so a Kismesis project cannot be created in it");
                }
            }
            Err(_) => return eprintln!("Couldn't check if the given path is empty"),
        },
        Ok(_) => match fs::create_dir(&name) {
            Ok(_) => (),
            Err(_) => return eprintln!("Failed to create create the given path"),
        },
        Err(_) => return eprintln!("Couldn't check if the given path exists"),
    }

    match fs::create_dir(name.clone().join("input")) {
        Ok(_) => (),
        Err(_) => return eprintln!("Couldn't create the input folder for the project"),
    }

    match fs::create_dir(name.clone().join("output")) {
        Ok(_) => (),
        Err(_) => return eprintln!("Couldn't create the output folder for the project"),
    }

    match fs::create_dir(name.clone().join("templates")) {
        Ok(_) => (),
        Err(_) => return eprintln!("Couldn't create the folder for the project's templates"),
    }

    println!("Created all folders in the specified path");

    match fs::write(name.clone().join("templates/main.ks"), DEFAULT_TEMPLATE) {
        Ok(_) => (),
        Err(_) => return eprintln!("Failed to create default example template"),
    }

    match fs::write(name.clone().join("input/index.ks"), DEFAULT_INDEX) {
        Ok(_) => (),
        Err(_) => return eprintln!("Failed to create default example input file"),
    }

    match ron::ser::to_string_pretty(&Settings::default(), DEFAULT_PRETTY_SETTINGS.clone()) {
        Ok(settings) => match fs::write(name.join(".kismet"), settings) {
            Ok(_) => (),
            Err(_) => return eprintln!("Failed to create default example input file"),
        },
        Err(_) => return eprintln!("Failed to create .kismet file contents from default settings"),
    }

    println!("Created project! Enter the respective folder if you're not already in it, and run `kismesis build`")
}

enum CouldntParseSettings {
    CouldntRead,
    CouldntParse(SpannedError),
}

fn get_settings() -> Result<Settings, CouldntParseSettings> {
    let Ok(settings_string) = std::fs::read_to_string(".kismet") else {
        return Err(CouldntParseSettings::CouldntRead);
    };

    let settings_string = settings_string.trim();

    if settings_string.is_empty() {
        let settings = Settings::default();
        match ron::ser::to_string_pretty(&settings, DEFAULT_PRETTY_SETTINGS.clone()) {
            Ok(settings) => match fs::write(".kismet", settings) {
                Ok(_) => (),
                Err(_) => eprintln!("Failed to create default example input file"),
            },
            Err(_) => eprintln!("Failed to create .kismet file contents from default settings"),
        }
        Ok(settings)
    } else {
        match ron::from_str::<Settings>(settings_string) {
            Ok(settings) => Ok(settings),
            Err(error) => Err(CouldntParseSettings::CouldntParse(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::set_current_dir;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    use kismesis::html;
    use kismesis::Kismesis;

    use crate::compile;
    use crate::compile::report_errors;
    use crate::get_settings;
    use crate::CouldntParseSettings;
    use crate::DEFAULT_INDEX;
    use crate::DEFAULT_TEMPLATE;

    #[test]
    fn test_default_files() {
        let mut errors = vec![];
        let mut engine = Kismesis::new();
        match engine.register_str(DEFAULT_TEMPLATE) {
            Ok(template) => {
                let template = engine.register_template(template);
                match engine.register_str(DEFAULT_INDEX) {
                    Ok(mut input) => {
                        input.template = Some(template);
                        println!("{:#?}", html::compile(&input, &engine));
                    }
                    Err(x) => errors.push(x.into()),
                }
            }
            Err(x) => errors.push(x.into()),
        }
        report_errors(&errors, &engine);
        assert!(errors.is_empty())
    }

    #[test]
    fn test_example_files() {
        let mut fail = false;
        let examples_path =
            PathBuf::from_str("examples").expect("Couldn't create examples path from string");
        for dir in
            fs::read_dir(examples_path).expect("Error creating dirs iterator from examples path")
        {
            let dir = dir.expect("Error getting the DirEntry");
            let path = dir.path();
            set_current_dir(&path).expect("Error setting current dir to path");
            let settings = match get_settings() {
                Ok(x) => x,
                Err(CouldntParseSettings::CouldntRead) => {
                    fail = true;
                    eprintln!("Couldn't read .kismet file");
                    continue;
                }
                Err(CouldntParseSettings::CouldntParse(error)) => {
                    fail = true;
                    eprintln!("Couldn't parse .kismet file: {error}");
                    continue;
                }
            };
            let mut engine = kismesis::Kismesis::new();
            engine.settings = settings;
            match compile::compile_project(&mut engine) {
                Ok(()) => (),
                Err(errors) => {
                    eprintln!("Errors for test in {}", path.display());
                    report_errors(&errors, &engine);
                    fail = true;
                }
            }
            set_current_dir("../../").expect("Error returnting to main dir");
        }
        if fail {
            panic!();
        }
    }
}
