# FriCAS Tauri Frontend

This project intends to provide a simple interactive frontend for FriCAS by using Tauri, React and MathJax to show [beautiful and accessible math](https://www.mathjax.org/).

## About FriCAS

[FriCAS](https://fricas.github.io/index.html) is an advanced computer algebra system.

It is freely available under a [modified BSD license](https://github.com/fricas/fricas/blob/master/LICENSE.txt).

[FriCAS](https://fricas.github.io/index.html) is a fork (2007) of the [Axiom](http://axiom-developer.org/) computer algebra system.

## Usage Tips

- You need to have `fricas` available in your `PATH` to use this project.
- Example: `brew install fricas` to install `fricas` on macOS.
- Binary releases are not available yet. You need to build it from source at the moment.

## Build from Source

```bash
git clone https://github.com/rn7s2/fricas-tauri
cd fricas-tauri
pnpm install
pnpm tauri build
```

Find the built application under `src-tauri/target/release/`.

## Development

This project uses [Tauri](https://tauri.app/) + [React](https://react.dev/) + [Typescript](https://www.typescriptlang.org/) to create cross-platform applications.

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
