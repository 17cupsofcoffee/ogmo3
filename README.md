# ogmo3

[![Build Status](https://img.shields.io/github/workflow/status/17cupsofcoffee/ogmo3/CI%20Build/main)](https://github.com/17cupsofcoffee/ogmo3/actions?query=branch%3Amain)
[![Crates.io](https://img.shields.io/crates/v/ogmo3.svg)](https://crates.io/crates/ogmo3)
[![Documentation](https://docs.rs/ogmo3/badge.svg)](https://docs.rs/ogmo3)
[![License](https://img.shields.io/crates/l/ogmo3.svg)](LICENSE)

`ogmo3` is a Rust crate for reading and writing [Ogmo Editor 3](https://ogmo-editor-3.github.io/) projects and levels.

It is modelled loosely off the API for Haxe's [`ogmo-3-lib`](https://github.com/Ogmo-Editor-3/ogmo-3-lib) (for now, at least), and aims to provide reasonably type-safe access to the entirety of Ogmo Editor 3.3.0's JSON schema.

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

## Caveats

* This crate does _not_ provide a runtime or renderer, but should make it easier to create one tailored to your project.
* If you deserialize a project/level and then reserialize it, there is no guarentee that the formatting/ordering of fields will be retained, as this would increase the complexity of the library significantly. However, there should never be any loss of data (and if there is, please file a bug report)!
* This crate has only been tested with Ogmo Editor 3.3.0 - data from earlier versions will likely fail to parse due to missing fields. 

## License

This project is licensed under the terms of the [MIT License](./LICENSE).