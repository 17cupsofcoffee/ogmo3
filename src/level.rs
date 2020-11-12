//! Functions and types for parsing Ogmo levels.

use std::fs;
use std::path::{Path, PathBuf};

use either::Either;
use hashbrown::HashMap;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

use crate::{Error, Vec2};

/// A dynamically typed value.
///
/// As Ogmo's level format does not store the type alongside the value,
/// it is not possible for this enum to specify the exact type of the
/// original value template.
#[derive(Clone, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    /// The version of Ogmo used to export this level.
    pub ogmo_version: String,

    /// The width of the level.
    pub width: f32,

    /// The height of the level.
    pub height: f32,

    /// The offset of the level on the X axis. Useful for loading multiple chunked levels.
    pub offset_x: f32,

    /// The offset of the level on the Y axis. Useful for loading multiple chunked levels.
    pub offset_y: f32,

    /// The level's custom values.
    #[serde(default)]
    pub values: HashMap<String, Value>,

    /// The layers that make up the level.
    pub layers: Vec<Layer>,
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

    /// Writes the Ogmo level to a JSON string.
    ///
    /// # Errors
    ///
    /// * `Error::Json` will be returned if serialization fails.
    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(Error::Json)
    }

    /// Writes the Ogmo level to a pretty-printed JSON string.
    ///
    /// # Errors
    ///
    /// * `Error::Json` will be returned if serialization fails.
    pub fn to_json_pretty(&self) -> Result<String, Error> {
        serde_json::to_string_pretty(self).map_err(Error::Json)
    }
}

/// An entity instance.
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,

    /// The width of the entity.
    /// Will only be present if the entity template was defined as resizable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,

    /// The X origin of the entity.
    /// Will only be present if the entity template defined an origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_x: Option<f32>,

    /// The Y origin of the entity.
    /// Will only be present if the entity template defined an origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_y: Option<f32>,

    /// The rotation of the entity.
    /// Will only be present if the entity template was defined as rotatable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f32>,

    /// Whether the entity is flipped on the X axis.
    /// Will only be present if the entity template was defined as X-flippable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flipped_x: Option<bool>,

    /// Whether the entity is flipped on the Y axis.
    /// Will only be present if the entity template was defined as Y-flippable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flipped_y: Option<bool>,

    /// The entity's nodes.
    /// Will only be present if the entity template was defined as having nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Vec2<f32>>>,

    /// The entity's custom values.
    /// Will only be present if the entity template was defined as having custom values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<HashMap<String, Value>>,
}

/// A decal instance.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Decal {
    /// The X position of the decal.
    pub x: f32,

    /// The Y position of the decal.
    pub y: f32,

    /// The scale of the decal on the X axis.
    /// Will only be present if the decal template was defined as scalable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_x: Option<f32>,

    /// The scale of the decal on the Y axis.
    /// Will only be present if the decal template was defined as scalable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_y: Option<f32>,

    /// The rotation of the decal.
    /// Will only be present if the decal template was defined as rotatable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f32>,

    /// The name of the decal's texture.
    pub texture: String,

    /// Custom values associated with the decal.
    pub values: HashMap<String, Value>,
}

/// A layer instance.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Layer {
    /// A tile layer.
    Tile(TileLayer),

    /// A tile co-ords layer.
    TileCoords(TileCoordsLayer),

    /// A grid layer.
    Grid(GridLayer),

    /// An entity layer.
    Entity(EntityLayer),

    /// A decal layer.
    Decal(DecalLayer),
}

impl Layer {
    /// Gets the name of the layer.
    pub fn name(&self) -> &str {
        match self {
            Layer::Tile(data) => &data.name,
            Layer::TileCoords(data) => &data.name,
            Layer::Grid(data) => &data.name,
            Layer::Entity(data) => &data.name,
            Layer::Decal(data) => &data.name,
        }
    }
}

/// A tile layer.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TileLayer {
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

    /// The name of the tileset used for this layer.
    pub tileset: String,

    /// The tile data.
    ///
    /// You may want to use the `unpack` method rather than accessing this directly.
    #[serde(flatten)]
    pub data: TileLayerStorage,
}

impl TileLayer {
    /// Unpack the tile data from the layer.
    pub fn unpack(&self) -> impl Iterator<Item = Tile> + '_ {
        match &self.data {
            TileLayerStorage::Data(data) => {
                Either::Left(data.iter().enumerate().map(move |(i, &v)| {
                    let grid_x = i as i32 % self.grid_cells_x;
                    let grid_y = i as i32 / self.grid_cells_x;

                    let pixel_x = grid_x * self.grid_cell_width;
                    let pixel_y = grid_y * self.grid_cell_height;

                    let id = if v == -1 { None } else { Some(v) };

                    Tile {
                        id,
                        grid_position: Vec2 {
                            x: grid_x,
                            y: grid_y,
                        },
                        pixel_position: Vec2 {
                            x: pixel_x,
                            y: pixel_y,
                        },
                    }
                }))
            }

            TileLayerStorage::Data2D(data) => {
                Either::Right(data.iter().enumerate().flat_map(move |(y, row)| {
                    row.iter().enumerate().map(move |(x, &v)| {
                        let grid_x = x as i32;
                        let grid_y = y as i32;

                        let pixel_x = grid_x * self.grid_cell_width;
                        let pixel_y = grid_y * self.grid_cell_height;

                        let id = if v == -1 { None } else { Some(v) };

                        Tile {
                            id,
                            grid_position: Vec2 {
                                x: grid_x,
                                y: grid_y,
                            },
                            pixel_position: Vec2 {
                                x: pixel_x,
                                y: pixel_y,
                            },
                        }
                    })
                }))
            }
        }
    }
}

/// An individual tile, unpacked from a `TileLayer`.
#[derive(Copy, Clone, Debug)]
pub struct Tile {
    /// The ID of the tile in the tileset.
    ///
    /// If the tile is empty, this will be `None`.
    pub id: Option<i32>,

    /// The position of the tile in grid co-ordinates.
    pub grid_position: Vec2<i32>,

    /// The position of the tile in pixel co-ordinates.
    pub pixel_position: Vec2<i32>,
}

/// Tile data from a `TileLayer`.
#[derive(Clone, Debug, Deserialize)]
pub enum TileLayerStorage {
    /// A flat list of tile IDs.
    ///
    /// Each value corresponds to the ID of a tile in a tileset (with `0` being the
    /// top left, and moving left to right, top to bottom).
    ///
    /// Empty tiles are represented by a `-1`.
    #[serde(rename = "data")]
    Data(Vec<i32>),

    /// A 2D list of tile IDs.
    ///
    /// Each value corresponds to the ID of a tile in a tileset (with `0` being the
    /// top left, and moving left to right, top to bottom).
    ///
    /// Empty tiles are represented by a `-1`.
    #[serde(rename = "data2D")]
    Data2D(Vec<Vec<i32>>),
}

impl Serialize for TileLayerStorage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TileLayerStorage", 3)?;

        match self {
            TileLayerStorage::Data(data) => {
                state.serialize_field("data", &data)?;
                state.serialize_field("exportMode", &0)?;
                state.serialize_field("arrayMode", &0)?;
            }
            TileLayerStorage::Data2D(data) => {
                state.serialize_field("data2D", &data)?;
                state.serialize_field("exportMode", &0)?;
                state.serialize_field("arrayMode", &1)?;
            }
        }

        state.end()
    }
}

/// A tile co-ords layer.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TileCoordsLayer {
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

    /// The name of the tileset used for this layer.
    pub tileset: String,

    /// The tile data.
    ///
    /// You may want to use the `unpack` method rather than accessing this directly.
    #[serde(flatten)]
    pub data: TileCoordsLayerStorage,
}

impl TileCoordsLayer {
    /// Unpack the tile data from the layer.
    pub fn unpack(&self) -> impl Iterator<Item = TileCoords> + '_ {
        match &self.data {
            TileCoordsLayerStorage::DataCoords(data) => {
                Either::Left(data.iter().enumerate().map(move |(i, coords)| {
                    let grid_x = i as i32 % self.grid_cells_x;
                    let grid_y = i as i32 / self.grid_cells_x;

                    let pixel_x = grid_x * self.grid_cell_width;
                    let pixel_y = grid_y * self.grid_cell_height;

                    let (grid_coords, pixel_coords) = if coords[0] == -1 {
                        (None, None)
                    } else {
                        let grid_u = coords[0];
                        let grid_v = coords[1];

                        let pixel_u = grid_u * self.grid_cell_width;
                        let pixel_v = grid_v * self.grid_cell_height;

                        (
                            Some(Vec2 {
                                x: grid_u,
                                y: grid_v,
                            }),
                            Some(Vec2 {
                                x: pixel_u,
                                y: pixel_v,
                            }),
                        )
                    };

                    TileCoords {
                        grid_coords,
                        pixel_coords,
                        grid_position: Vec2 {
                            x: grid_x,
                            y: grid_y,
                        },
                        pixel_position: Vec2 {
                            x: pixel_x,
                            y: pixel_y,
                        },
                    }
                }))
            }

            TileCoordsLayerStorage::DataCoords2D(data) => {
                Either::Right(data.iter().enumerate().flat_map(move |(y, row)| {
                    row.iter().enumerate().map(move |(x, coords)| {
                        let grid_x = x as i32;
                        let grid_y = y as i32;

                        let pixel_x = grid_x * self.grid_cell_width;
                        let pixel_y = grid_y * self.grid_cell_height;

                        let (grid_coords, pixel_coords) = if coords[0] == -1 {
                            (None, None)
                        } else {
                            let grid_u = coords[0];
                            let grid_v = coords[1];

                            let pixel_u = grid_u * self.grid_cell_width;
                            let pixel_v = grid_v * self.grid_cell_height;

                            (
                                Some(Vec2 {
                                    x: grid_u,
                                    y: grid_v,
                                }),
                                Some(Vec2 {
                                    x: pixel_u,
                                    y: pixel_v,
                                }),
                            )
                        };

                        TileCoords {
                            grid_coords,
                            pixel_coords,
                            grid_position: Vec2 {
                                x: grid_x,
                                y: grid_y,
                            },
                            pixel_position: Vec2 {
                                x: pixel_x,
                                y: pixel_y,
                            },
                        }
                    })
                }))
            }
        }
    }
}

/// An individual tile, unpacked from a `TileCoordsLayer`.
#[derive(Copy, Clone, Debug)]
pub struct TileCoords {
    /// The position of the tile in the tileset, in grid co-ordinates.
    ///
    /// If the tile is empty, this will be `None`.
    pub grid_coords: Option<Vec2<i32>>,

    /// The position of the tile in the tileset, in pixel co-ordinates.
    ///
    /// If the tile is empty, this will be `None`.
    pub pixel_coords: Option<Vec2<i32>>,

    /// The position of the tile in grid co-ordinates.
    pub grid_position: Vec2<i32>,

    /// The position of the tile in pixel co-ordinates.
    pub pixel_position: Vec2<i32>,
}

/// Tile data from a `TileCoordsLayer`.
#[derive(Clone, Debug, Deserialize)]
pub enum TileCoordsLayerStorage {
    /// A flat list of tile co-ords.
    ///
    /// Each value corresponds to the X and Y co-ordinate of a tile in a tileset. The
    /// values are cell-based, rather than pixel-based - multiply by `grid_cell_width`
    /// to get the pixel position.
    ///
    /// Empty tiles are represented by a `[-1]`.
    #[serde(rename = "dataCoords")]
    DataCoords(Vec<Vec<i32>>),

    /// A 2D list of tile co-ords.
    ///
    /// Each value corresponds to the X and Y co-ordinate of a tile in a tileset. The
    /// values are cell-based, rather than pixel-based - multiply by `grid_cell_width`
    /// to get the pixel position.
    ///
    /// Empty tiles are represented by a `[-1]`.
    #[serde(rename = "dataCoords2D")]
    DataCoords2D(Vec<Vec<Vec<i32>>>),
}

impl Serialize for TileCoordsLayerStorage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TileCoordsLayerStorage", 3)?;

        match self {
            TileCoordsLayerStorage::DataCoords(data) => {
                state.serialize_field("dataCoords", &data)?;
                state.serialize_field("exportMode", &1)?;
                state.serialize_field("arrayMode", &0)?;
            }
            TileCoordsLayerStorage::DataCoords2D(data) => {
                state.serialize_field("dataCoords2D", &data)?;
                state.serialize_field("exportMode", &1)?;
                state.serialize_field("arrayMode", &1)?;
            }
        }

        state.end()
    }
}

/// A grid layer.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridLayer {
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

    /// The grid data.
    ///
    /// You may want to use the `unpack` method rather than accessing this directly.
    #[serde(flatten)]
    pub data: GridLayerStorage,
}

/// Grid data from a `GridLayer`.
#[derive(Clone, Debug, Deserialize)]
pub enum GridLayerStorage {
    /// A flat list of string data.
    ///
    /// By default, `"0"` means 'empty', but this is customizable in the editor.
    #[serde(rename = "grid")]
    Grid(Vec<String>),

    /// A 2D list of string data.
    ///
    /// By default, `"0"` means 'empty', but this is customizable in the editor.
    #[serde(rename = "grid2D")]
    Grid2D(Vec<Vec<String>>),
}

impl Serialize for GridLayerStorage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GridLayerStorage", 2)?;

        match self {
            GridLayerStorage::Grid(data) => {
                state.serialize_field("grid", &data)?;
                state.serialize_field("arrayMode", &0)?;
            }
            GridLayerStorage::Grid2D(data) => {
                state.serialize_field("grid2D", &data)?;
                state.serialize_field("arrayMode", &1)?;
            }
        }

        state.end()
    }
}

/// An individual grid cell, unpacked from a `GridLayer`.
#[derive(Copy, Clone, Debug)]
pub struct GridCell<'a> {
    /// The value of the grid cell.
    ///
    /// By default, `"0"` means 'empty', but this is customizable in the editor.
    pub value: &'a str,

    /// The position of the cell in grid co-ordinates.
    pub grid_position: Vec2<i32>,

    /// The position of the cell in pixel co-ordinates.
    pub pixel_position: Vec2<i32>,
}

impl GridLayer {
    /// Unpack the grid data from the layer.
    pub fn unpack(&self) -> impl Iterator<Item = GridCell<'_>> + '_ {
        match &self.data {
            GridLayerStorage::Grid(data) => {
                Either::Left(data.iter().enumerate().map(move |(i, value)| {
                    let grid_x = i as i32 % self.grid_cells_x;
                    let grid_y = i as i32 / self.grid_cells_x;

                    let pixel_x = grid_x * self.grid_cell_width;
                    let pixel_y = grid_y * self.grid_cell_height;

                    GridCell {
                        value,
                        grid_position: Vec2 {
                            x: grid_x,
                            y: grid_y,
                        },
                        pixel_position: Vec2 {
                            x: pixel_x,
                            y: pixel_y,
                        },
                    }
                }))
            }

            GridLayerStorage::Grid2D(data) => {
                Either::Right(data.iter().enumerate().flat_map(move |(y, row)| {
                    row.iter().enumerate().map(move |(x, value)| {
                        let grid_x = x as i32;
                        let grid_y = y as i32;

                        let pixel_x = grid_x * self.grid_cell_width;
                        let pixel_y = grid_y * self.grid_cell_height;

                        GridCell {
                            value,
                            grid_position: Vec2 {
                                x: grid_x,
                                y: grid_y,
                            },
                            pixel_position: Vec2 {
                                x: pixel_x,
                                y: pixel_y,
                            },
                        }
                    })
                }))
            }
        }
    }
}

/// An entity layer.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityLayer {
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

    /// Entity data.
    pub entities: Vec<Entity>,
}

/// A decal layer.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecalLayer {
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

    /// Decal data.
    pub decals: Vec<Decal>,

    /// The path containing the decal images, relative to the project.
    pub folder: PathBuf,
}
