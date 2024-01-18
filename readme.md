# KISMESIS

Kismesis is a static site generator with macros, plugins and spite, encouraging everybody to make static sites that are semantic and accessible.

## Features
- **Only one language for your content:** No need to use HTML for templates, Markdown for articles, and maybe something else for a third thing. The Kismesis markup language is designed to complement the static site generator while being concise and easier to type. You're still going to need CSS and Javascript, though.
- **Compile-time errors:** Kismesis will give you a visual report of your errors, signaling where they happened, similar to what the Rust compiler does.
- **Macros:** Custom HTML tags you can define in order to use them anywhere to make tasks less repetitive.
- **(UNSTABLE!) Plugins:** A way for users to add new features to Kismesis.
- **(TO DO!) A Standard Library:** Ever miss the `<marquee>` tag? In Kismesis, you're not allowed to use deprecated tags, however it will come with a `<marquee?>` plugin, which will aim to replicate the behavior using only widely supported featues.

# How To Use
- Download one of the builds from the releases and add it to your `$PATH` environment variable. I recommend that you rename the executable to `kismesis` instead of `kismesis-ssg`
- To get started, run `kismesis new [project_name]`. This will create a new folder for your project. 
- Run `kismesis build` to build a project.

# Compiling
Simply install the rust toolchain, clone this repository and do `cargo build`. It literally is that simple. This is Rust we're talking about.

You don't need to clone the repository - if you just wish to compile binaries locally, you can always run `cargo install kismesis-ssg`.

## Roadmap

- Make a TUI mode for the program
- Make a manual that is visible from this TUI
- Make a plugin manager
- Allow plugins to create new subcommands for the CLI

Check the [Kismesis Engine readme](https://github.com/lilith-in-starlight/kismesis) for a roadmap pertaining to the language and the generator's engine.

## FAQ
FAQ stands for both Forwardly Anticipated Questions and Frequently-ish Asked Questions.

### Why do you plan to error when using `<div>`?
- The reason for this is that we often utilize divisions as a catch-all, instead of utilizing more semantic alternatives like `header`, `footer`, `main`, `section`, and even `button`. `<container>` will get compiled as `<div>` in case you really know there is no better alternative. Think of it the same way you think of Rust's `unsafe`.
  - No, `<button>` is not that much harder to style than a div.
  
For questions about the Kismesis Language or its Engine, check [its respective readme](https://github.com/lilith-in-starlight/kismesis).
