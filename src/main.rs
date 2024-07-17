mod compile;

use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};

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
}

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

fn main() {
    let cli = Kismesis::parse();

    match cli.command {
        Some(Commands::Build) => compile::compile_project(),
        Some(Commands::New { name }) => new(name.unwrap_or(".".to_string())),
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

    match fs::write(name.join("input/index.ks"), DEFAULT_INDEX) {
        Ok(_) => (),
        Err(_) => return eprintln!("Failed to create default example input file"),
    }

    println!("Created project! Enter the respective folder if you're not already in it, and run `kismesis build`")
}

#[cfg(test)]
mod tests {
    use kismesis::Kismesis;
    use kismesis::html;

    use crate::compile::report_errors;
    use crate::DEFAULT_INDEX;
    use crate::DEFAULT_TEMPLATE;

    #[test]
    fn test_default_files() {
        let mut errors = vec![];
		let mut engine = Kismesis::new();
		match engine
			.register_str(DEFAULT_TEMPLATE) {
		    Ok(template) => {
        		let template = engine.register_template(template);
        		match engine.register_str(DEFAULT_INDEX) {
        		    Ok(mut input) => {
                		input.template = Some(template);
                		println!("{:#?}", html::compile(&input, &engine));
        		    }
        		    Err(x) => errors.push(x.into()),
        		}
		    },
		    Err(x) => errors.push(x.into()),
		}
		report_errors(&errors, &engine);
		assert!(errors.is_empty())
    }
}
