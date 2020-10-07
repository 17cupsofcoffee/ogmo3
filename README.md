# ogmo3

`ogmo3` is a Rust crate for parsing projects and levels created with [Ogmo Editor 3](https://ogmo-editor-3.github.io/).

## Design Goals

* Where possible/sensible, the design should match [`ogmo-3-lib`](https://github.com/Ogmo-Editor-3/ogmo-3-lib) (the reference implementation of an Ogmo project parser), so that code can easily be translated.
* Where possible/sensible, the exposed structs should match the layout of the JSON, rather than trying to interpret them into a runtime format. Higher level tools may be provided in future as a layer on top.
* Use enums instead of `Option` to represent mutually exclusive fields.