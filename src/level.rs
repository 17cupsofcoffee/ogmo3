//! Functions and types for parsing Ogmo levels.

use std::fs;
use std::path::{Path, PathBuf};

use hashbrown::HashMap;
use serde::Deserialize;

use crate::{Error, Vec2};

/// A dynamically typed value.
///
/// As Ogmo's level format does not store the type alongside the value,
/// it is not possible for this enum to specify the exact type of the
/// original value template.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Value {
    /// A boolean value.
    Boolean(bool),

    /// A string value.
    String(String),

    /// A numeric value.
    ///
    /// This may have originally been an integer when set, but the Ogmo
    /// format does not provide enough information to figure that out
    /// without cross-referencing the project.
    Number(f32),
}

/// An Ogmo level.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    /// The width of the level.
    pub width: f32,

    /// The height of the level.
    pub height: f32,

    /// The offset of the level on the X axis. Useful for loading multiple chunked levels.
    pub offset_x: f32,

    /// The offset of the level on the Y axis. Useful for loading multiple chunked levels.
    pub offset_y: f32,

    /// The layers that make up the level.
    pub layers: Vec<Layer>,

    /// The level's custom values.
    #[serde(default)]
    pub values: HashMap<String, Value>,
}

impl Level {
    /// Parses an Ogmo level from a JSON string.
    ///
    /// # Errors
    ///
    /// * `Error::Json` will be returned if deserialization fails.
    pub fn from_json(s: &str) -> Result<Level, Error> {
        serde_json::from_str(s).map_err(Error::Json)
    }

    /// Parses an Ogmo level from a file.
    ///
    /// # Errors
    ///
    /// * `Error::Io` will be returned if the file cannot be read.
    /// * `Error::Json` will be returned if deserialization fails.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Level, Error> {
        let json = fs::read_to_string(path).map_err(Error::Io)?;
        Level::from_json(&json)
    }
}

/// An entity instance.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    /// The entity's name.
    pub name: String,

    /// The entity's ID.
    pub id: i32,

    /// The unique export ID of the entity.
    #[serde(rename = "_eid")]
    pub export_id: String,

    /// The X position of the entity.
    pub x: f32,

    /// The Y position of the entity.
    pub y: f32,

    /// The width of the entity.
    /// Will only be present if the entity template was defined as resizable.
    pub width: Option<f32>,

    /// The width of the entity.
    /// Will only be present if the entity template was defined as resizable.
    pub height: Option<f32>,

    /// The X origin of the entity.
    /// Will only be present if the entity template defined an origin.
    pub origin_x: Option<f32>,

    /// The Y origin of the entity.
    /// Will only be present if the entity template defined an origin.
    pub origin_y: Option<f32>,

    /// The rotation of the entity.
    /// Will only be present if the entity template was defined as rotatable.
    pub rotation: Option<f32>,

    /// Whether the entity is flipped on the X axis.
    /// Will only be present if the entity template was defined as X-flippable.
    pub flipped_x: Option<bool>,

    /// Whether the entity is flipped on the Y axis.
    /// Will only be present if the entity template was defined as Y-flippable.
    pub flipped_y: Option<bool>,

    /// The entity's nodes.
    /// Will only be present if the entity template was defined as having nodes.
    pub nodes: Option<Vec<Vec2<f32>>>,

    /// The entity's custom values.
    /// Will only be present if the entity template was defined as having custom values.
    pub values: Option<HashMap<String, Value>>,
}

/// A decal instance.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decal {
    /// The X position of the decal.
    pub x: f32,

    /// The Y position of the decal.
    pub y: f32,

    /// The name of the decal's texture.
    pub texture: String,

    /// The rotation of the decal.
    /// Will only be present if the decal template was defined as rotatable.
    pub rotation: Option<f32>,

    /// The scale of the decal on the X axis.
    /// Will only be present if the decal template was defined as scalable.
    pub scale_x: Option<f32>,

    /// The scale of the decal on the Y axis.
    /// Will only be present if the decal template was defined as scalable.
    pub scale_y: Option<f32>,
}

/// A layer instance.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    /// The name of the layer.
    pub name: String,

    /// The unique export ID of the entity.
    #[serde(rename = "_eid")]
    pub export_id: String,

    /// The layer's offset on the X axis.
    pub offset_x: f32,

    /// The layer's offset on the Y axis.
    pub offset_y: f32,

    /// The width of the layer's grid cells.
    pub grid_cell_width: i32,

    /// The height of the layer's grid cells.
    pub grid_cell_height: i32,

    /// The number of grid cells on the X axis.
    pub grid_cells_x: i32,

    /// The number of grid cells on the Y axis.
    pub grid_cells_y: i32,

    /// Data specific to certain layer types.
    #[serde(flatten)]
    pub data: LayerData,
}

/// Data specific to certain layer types.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum LayerData {
    /// Data specific to tile layers with 1D storage.
    Tile(LayerTileData),

    /// Data specific to tile layers with 2D storage.
    Tile2D(LayerTile2DData),

    /// Data specific to tile co-ord layers with 1D storage.
    TileCoords(LayerTileCoordsData),

    /// Data specific to tile co-ord layers with 2D storage.
    TileCoords2D(LayerTileCoords2DData),

    /// Data specific to grid layers with 1D storage.
    Grid(LayerGridData),

    /// Data specific to grid layers with 2D storage.
    Grid2D(LayerGrid2DData),

    /// Data specific to entity layers.
    Entity(LayerEntityData),

    /// Data specific to decal layers.
    Decal(LayerDecalData),
}

/// Data specific to tile layers with 1D storage.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerTileData {
    /// The tile data, stored as a flat list of IDs.
    ///
    /// Each value corresponds to the ID of a tile in a tileset (with `0` being the
    /// top left, and moving left to right, top to bottom).
    ///
    /// Empty tiles are represented by a `-1`.
    pub data: Vec<i32>,

    /// The name of the tileset used for this layer.
    pub tileset: String,
}

/// Data specific to tile layers with 2D storage.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerTile2DData {
    /// The tile data, stored as a 2D list of IDs.
    ///
    /// Each value corresponds to the ID of a tile in a tileset (with `0` being the
    /// top left, and moving left to right, top to bottom).
    ///
    /// Empty tiles are represented by a `-1`.
    #[serde(rename = "data2D")]
    pub data_2d: Vec<Vec<i32>>,

    /// The name of the tileset used for this layer.
    pub tileset: String,
}

/// Data specific to tile co-ord layers with 1D storage.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerTileCoordsData {
    /// The tile data, stored as a flat list of tile co-ordinates.
    ///
    /// Each value corresponds to the X and Y co-ordinate of a tile in a tileset. The
    /// values are cell-based, rather than pixel-based - multiply by `grid_cell_width`
    /// to get the pixel position.
    ///
    /// Empty tiles are represented by a `[-1]`.
    pub data_coords: Vec<Vec<i32>>,

    /// The name of the tileset used for this layer.
    pub tileset: String,
}

/// Data specific to tile co-ord layers with 2D storage.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerTileCoords2DData {
    /// The tile data, stored as a 2D list of tile co-ordinates.
    ///
    /// Each value corresponds to the X and Y co-ordinate of a tile in a tileset. The
    /// values are cell-based, rather than pixel-based - multiply by `grid_cell_width`
    /// to get the pixel position.
    ///
    /// Empty tiles are represented by a `[-1]`.
    #[serde(rename = "dataCoords2D")]
    pub data_coords_2d: Vec<Vec<Vec<i32>>>,

    /// The name of the tileset used for this layer.
    pub tileset: String,
}

/// Data specific to grid layers with 1D storage.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerGridData {
    /// The grid data, stored as a flat list.
    ///
    /// Each value is an arbitary string - by default, `0` means 'empty', but this is
    /// customizable in the editor.
    pub grid: Vec<String>,
}

/// Data specific to grid layers with 2D storage.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerGrid2DData {
    /// The grid data, stored as a 2D list.
    ///
    /// Each value is an arbitary string - by default, `0` means 'empty', but this is
    /// customizable in the editor.
    #[serde(rename = "grid2D")]
    pub grid_2d: Vec<Vec<String>>,
}

/// Data specific to entity layers.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerEntityData {
    /// The entity data.
    pub entities: Vec<Entity>,
}

/// Data specific to decal layers.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerDecalData {
    /// The entity data.
    pub decals: Vec<Decal>,

    /// The path containing the decal images, relative to the project.
    pub folder: PathBuf,
}
