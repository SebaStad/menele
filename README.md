# Tauri + Yew

This template should help get you started developing with Tauri and Yew.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


## Stuff from install:

### Apt gets
```
sudo apt-get update
sudo apt-get upgrade
sudo apt-get install libssl-dev pkg-config build-essential libwebkit2gtk-4.1-dev   build-essential   curl   wget   file   libxdo-dev   libssl-dev   libayatana-appindicator3-dev   librsvg2-dev clang nsis llvm
```
### After template:
Template created!

Your system is missing dependencies (or they do not exist in $PATH):
```
╭────────────────────┬───────────────────────────────────────────────────────────╮
│ Tauri CLI          │ Run `cargo install tauri-cli --version '^2.0.0' --locked` │
├────────────────────┼───────────────────────────────────────────────────────────┤
│ Trunk              │ Run `cargo install trunk --locked`                        │
├────────────────────┼───────────────────────────────────────────────────────────┤
│ wasm32 target      │ Run `rustup target add wasm32-unknown-unknown`            │
├────────────────────┼───────────────────────────────────────────────────────────┤
│ webkit2gtk & rsvg2 │ Visit https://tauri.app/guides/prerequisites/#linux       │
╰────────────────────┴───────────────────────────────────────────────────────────╯
```

Make sure you have installed the prerequisites for your OS: https://tauri.app/start/prerequisites/, then run:
```
  cd menele
  cargo tauri android init
```

For Desktop development, run:
```
  cargo tauri dev
```
For Android development, run:
```
  cargo tauri android dev
```
For building app on Linux, run:
```
  cargo tauri build
```

### Build for windows:
https://v1.tauri.app/v1/guides/building/cross-platform/
```
rustup target add x86_64-pc-windows-msvc
cargo install --locked cargo-xwin
cargo tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc
```

### Quick css check?

```
trunk serve
```

Trunk is rust webasm stuff thingy


### yew vs yew-router

Docs are lieing, dont use git version but 0.18 version in toml for yew-router

## Settings Ideas

* Light mode / Dark Mode
* Farbenanpassung
* PDF to image
  * Implement image as code
* Back to default Settings


* Adjust variables in Props for formatting
* Fix Preview html codegen
* Select all button in html preview
* Copy to clipboard button in html preview
* Check loading is working correctly


## Todo's

* restructure code
  * files
  * functions/methods
* Fix format of live Preview for small screens
* Add preview in separate router

## Command Lines:

Current lines of RustCode:

```
wc -l $(git ls-files | grep '.*\.rs')
wc -l $(git ls-files | grep '.*\.html')

```