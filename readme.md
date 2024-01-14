# KISMESIS

Kismesis is a static site generator with macros, plugins and spite, encouraging everybody to make static sites that are semantic and accessible.

## Features
- **Only one language to type in:** No need to use HTML for one thing, Markdown for another, and maybe something else. The Kiss markup language is designed to complement the static site generator while being concise and easier to type.
- **Compile-time errors:** Aren't you tired of mistyping your pages and realizing later that they've been rendered all wrong? Kismesis will give you a visual report of your erros signaling where they happened, similar to what the Rust compiler does.
- **Macros:** Custom HTML tags you can define in order to use them anywhere. You can supply custom arguments to them.
- **(UNSTABLE!) Plugins:** Plugins allow you to add features and new syntax to the markup language. They use WASM.
- **(TO DO!) A Standard Library:** Ever miss the <marquee> tag? In Kismesis, you're not allowed to use deprecated tags, however it will come with a <marquee?> plugin, which will aim to replicate the behavior using only widely supported featues.

# How To Use
- Download one of the builds from the releases and add it to your `$PATH` environment variable. I recommend that you rename the executable to `kismesis` instead of `kismesis-ssg`
- To get started, run `kismesis new --name [project_name]`. This will create a new folder for your project. 
- Run `kismesis build` to build a project.

## Roadmap

- Make a TUI mode for that program
- Make a manual that is visible from this TUI
- Make a plugin manager
- Allow plugins to create new subcommands for the CLI

Check the [Kismesis Engine readme](https://github.com/lilith-in-starligh/kismesis) for a roadmap pertaining to the language and the generator's engine.

## FAQ
FAQ stands for both Forwardly Anticipated Questions and Frequently-ish Asked Questions.

### Why do you plan to error when using `<div>`?
- The reason for this is that we often utilize divisions as a catch-all, instead of utilizing more semantic alternatives like `header`, `footer`, `main`, `section`, and even `button`. `<container>` will get compiled as `<div>` in case you really know there is no better alternative. Think of it the same way you think of Rust's `unsafe`.
  - No, `<button>` is not that much harder to style than a div.
  
For questions about the Kismesis Language or its Engine, check [its respective readme](https://github.com/lilith-in-starligh/kismesis)
