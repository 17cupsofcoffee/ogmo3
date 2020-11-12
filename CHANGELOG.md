# Changelog

All notable changes to this project will be documented in this file, following the format defined at keepachangelog.com. Where a change was contributed via a third-party pull request, the author will be credited.

All breaking changes will be explicitly labelled, to make it easier to assess the impact of upgrading.

This project adheres to Semantic Versioning.

## [0.1.0] - 2020-11-12

### Added

* Added serialization support.
* Added `name` getters to various structs.

## [0.0.4] - 2020-11-09

### Added 

* `unpack` methods have been added to `TileLayer`, `TileCoordsLayer` and `GridLayer`, which allow for reading tile and grid data without boilerplate.
* `Layer` and `Value` are now re-exported from the top level of the crate.

### Changed

* **Breaking:** `Layer`, `ValueTemplate` and `LayerTemplate` are now defined as enums instead of structs containing enums.
    * This creates some duplication between the different enum variants, but makes it easier to unpack the data. It also simplifies the implementation/usage of the new utility methods.
* **Breaking**: The different types of grid, tile and tile-coords layers have been replaced with single types with an enum defining the data storage.
* Improved documentation.

## [0.0.3] - 2020-10-14

### Added 

* `Tileset` now has a method for getting the co-ordinates of each tile in the tileset.

### Fixed

* Parsing a `Level` no longer fails if there are no custom values defined for levels in the project.

## [0.0.2] - 2020-10-07

### Fixed

* `Vec2`'s fields are now public.

## [0.0.1] - 2020-10-07

* Initial release!

[Upcoming]: https://github.com/17cupsofcoffee/ogmo3/compare/0.1.0..HEAD
[0.1.0]: https://github.com/17cupsofcoffee/ogmo3/compare/0.0.4..0.1.0
[0.0.4]: https://github.com/17cupsofcoffee/ogmo3/compare/0.0.3..0.0.4
[0.0.3]: https://github.com/17cupsofcoffee/ogmo3/compare/0.0.2..0.0.3
[0.0.2]: https://github.com/17cupsofcoffee/ogmo3/compare/0.0.1..0.0.2
[0.0.1]: https://github.com/17cupsofcoffee/ogmo3/compare/41a781f..0.0.1