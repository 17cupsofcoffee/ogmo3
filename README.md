# ogmo3

[![Build Status](https://img.shields.io/github/workflow/status/17cupsofcoffee/ogmo3/CI%20Build/main)](https://github.com/17cupsofcoffee/ogmo3/actions?query=branch%3Amain)
[![Crates.io](https://img.shields.io/crates/v/ogmo3.svg)](https://crates.io/crates/ogmo3)
[![Documentation](https://docs.rs/ogmo3/badge.svg)](https://docs.rs/ogmo3)
[![License](https://img.shields.io/crates/l/ogmo3.svg)](LICENSE)

`ogmo3` is a Rust crate for parsing projects and levels created with [Ogmo Editor 3](https://ogmo-editor-3.github.io/).

It is modelled loosely off the API for Haxe's [`ogmo-3-lib`](https://github.com/Ogmo-Editor-3/ogmo-3-lib) (for now, at least), and aims to provide type-safe access to the entirety of Ogmo's JSON schema.

This crate does _not_ provide a runtime or renderer, but should make it easier to create one tailored to your project.

## Installation


```toml
[dependencies]
ogmo3 = "0.0.4"
```

## Usage

```rust
use ogmo3::{Level, Project};

fn main() {
    let project = Project::from_file("./example.ogmo").unwrap();
    let level = Level::from_file("./levels/level.json").unwrap();
}
```

For a full example of how to interpret the data in an Ogmo project, see the [sample code](./examples/sample.rs).

## License

This project is licensed under the terms of the [MIT License](./LICENSE).