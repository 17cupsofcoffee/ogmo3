//! Functions and types for parsing Ogmo projects.

use std::fs;
use std::path::{Path, PathBuf};

use hashbrown::HashMap;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::{Error, Vec2};

/// An Ogmo project.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// The name of the Ogmo project.
    pub name: String,

    /// An array of paths that hold the project's levels.
    pub level_paths: Vec<PathBuf>,

    /// The project's background color.
    pub background_color: String,

    /// The color of the grid displayed in the editor.
    pub grid_color: String,

    /// Whether the project describes angles in radians or degrees.
    pub angles_radians: bool,

    /// The default exported file type of a level.
    pub default_export_mode: String,

    /// The maximum depth that the editor will search for levels in its file tree.
    pub directory_depth: i32,

    /// The default size of newly created levels in the editor.
    pub level_default_size: Vec2<i32>,

    /// The minimum size of a level.
    pub level_min_size: Vec2<i32>,

    /// The maximum size of a level.
    pub level_max_size: Vec2<i32>,

    /// The value templates for the project's levels.
    pub level_values: Vec<ValueTemplate>,

    /// The tags that can be attached to entities.
    pub entity_tags: Vec<String>,

    /// The project's layer templates.
    pub layers: Vec<LayerTemplate>,

    /// The project's entity templates.
    pub entities: Vec<EntityTemplate>,

    /// The project's tilesets.
    pub tilesets: Vec<Tileset>,

    /// The default grid size for newly created layers.
    pub layer_grid_default_size: Vec2<i32>,
}

impl Project {
    /// Parses an Ogmo project from a JSON string.
    ///
    /// # Errors
    ///
    /// * `Error::Json` will be returned if deserialization fails.
    pub fn from_json(s: &str) -> Result<Project, Error> {
        serde_json::from_str(s).map_err(Error::Json)
    }

    /// Parses an Ogmo project from a file.
    ///
    /// # Errors
    ///
    /// * `Error::Io` will be returned if the file cannot be read.
    /// * `Error::Json` will be returned if deserialization fails.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Project, Error> {
        let json = fs::read_to_string(path).map_err(Error::Io)?;
        Project::from_json(&json)
    }
}

/// A template for a value.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueTemplate {
    /// The name of the value.
    pub name: String,

    /// Data specific to certain value template types.
    #[serde(flatten)]
    pub data: ValueTemplateData,
}

/// Data specific to certain value template types.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "definition")]
pub enum ValueTemplateData {
    /// Data specific to boolean value templates.
    Boolean(ValueTemplateBooleanData),

    /// Data specific to color value templates.
    Color(ValueTemplateColorData),

    /// Data specific to enum value templates.
    Enum(ValueTemplateEnumData),

    /// Data specific to integer value templates.
    Integer(ValueTemplateIntegerData),

    /// Data specific to float value templates.
    Float(ValueTemplateFloatData),

    /// Data specific to string value templates.
    String(ValueTemplateStringData),

    /// Data specific to text value templates.
    Text(ValueTemplateTextData),
}

/// Data specific to boolean value templates.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueTemplateBooleanData {
    /// The default value.
    pub defaults: bool,
}

/// Data specific to color value templates.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueTemplateColorData {
    /// The default value.
    pub defaults: String,

    /// Whether the alpha component will be included in the color.
    pub include_alpha: bool,
}

/// Data specific to enum value templates.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueTemplateEnumData {
    /// The default value.
    pub defaults: i32,

    /// The available choices for the enum.
    pub choices: Vec<String>,
}

/// Data specific to integer value templates.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueTemplateIntegerData {
    /// The default value.
    pub defaults: i32,

    /// Whether the value is bounded with a min and/or max value.
    pub bounded: bool,

    /// The minimum value.
    pub min: i32,

    /// The maximum value.
    pub max: i32,
}

/// Data specific to float value templates.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueTemplateFloatData {
    /// The default value.
    pub defaults: f32,

    /// Whether the value is bounded with a min and/or max value.
    pub bounded: bool,

    /// The minimum value.
    pub min: f32,

    /// The maximum value.
    pub max: f32,
}

/// Data specific to string value templates.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueTemplateStringData {
    /// The default value.
    pub defaults: String,

    /// The maximum length.
    pub max_length: i32,

    /// Whether whitespace should be trimmed from the beginning and end of the string.
    pub trim_whitespace: bool,
}

/// Data specific to text value templates.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueTemplateTextData {
    /// The default value.
    pub defaults: String,
}

/// A template for a layer.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerTemplate {
    /// The name of the layer.
    pub name: String,

    /// The size of each cell in the layer's grid.
    pub grid_size: Vec2<i32>,

    /// The unique export ID of the layer.
    #[serde(rename = "exportID")]
    pub export_id: String,

    /// Data specific to certain layer template types.
    #[serde(flatten)]
    pub data: LayerTemplateData,
}

/// Data specific to certain layer template types.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum LayerTemplateData {
    /// Data specific to tile layer templates.
    Tile(LayerTemplateTileData),

    /// Data specific to grid layer templates.
    Grid(LayerTemplateGridData),

    /// Data specific to entity layer templates.
    Entity(LayerTemplateEntityData),

    /// Data specific to decal layer templates.
    Decal(LayerTemplateDecalData),
}

/// Data specific to tile layer templates.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerTemplateTileData {
    /// Whether the tile data is stored as IDs or co-oords.
    pub export_mode: ExportMode,

    /// Whether the tile data is stored as a 1D array or a 2D array.
    pub array_mode: ArrayMode,

    /// The default tileset for the layer.
    pub default_tileset: String,
}

/// Data specific to grid layer templates.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerTemplateGridData {
    /// Whether the tile data is stored as a 1D array or a 2D array.
    pub array_mode: ArrayMode,

    /// Descriptions for the available grid cells.
    pub legend: HashMap<String, String>,
}

/// Data specific to entity layer templates.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerTemplateEntityData {
    /// Tags that are required for an entity to be displayed on this layer.
    pub required_tags: Vec<String>,

    /// Tags that must not be present for an entity to be displayed on this layer.
    pub excluded_tags: Vec<String>,
}

/// Data specific to decal layer templates.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerTemplateDecalData {
    /// The path to search for decal images, relative to the project
    pub folder: PathBuf,

    /// Whether image sequences are included as available decals.
    pub include_image_sequence: bool,

    /// Whether this layer's decals are scalable.
    pub scaleable: bool,

    /// Whether this layer's decals are rotatable.
    pub rotatable: bool,

    /// Value templates associated with this decal layer.
    pub values: Vec<ValueTemplate>,
}

/// Defines whether tile data is stored as IDs or co-oords.
#[derive(Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ExportMode {
    /// The tile data is represented by IDs (counting left to right, top to bottom).
    Ids = 0,

    /// The tile data is represented by co-ordinates.
    Coords = 1,
}

/// Defines whether tile data is stored as a 1D array or a 2D array.
#[derive(Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ArrayMode {
    /// The tile data is stored in a 1D array.
    One = 0,

    /// The tile data is stored in a 2D array.
    Two = 1,
}

/// A template for an entity.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityTemplate {
    /// The name of the entity.
    pub name: String,

    /// The unique export ID of the entity.
    #[serde(rename = "exportID")]
    pub export_id: String,

    /// The maximum number of instances. 0 to ignore.
    pub limit: i32,

    /// The size of the entity.
    pub size: Vec2<f32>,

    /// The origin of the entity.
    pub origin: Vec2<f32>,

    /// Whether the entity is anchored to the origin.
    pub origin_anchored: bool,

    /// The shape of the entity.
    pub shape: Shape,

    /// The color of the entity's icon.
    pub color: String,

    /// Whether the icon should tile on the X axis.
    pub tile_x: bool,

    /// Whether the icon should tile on the Y axis.
    pub tile_y: bool,

    /// The tiled icon size.
    pub tile_size: Vec2<f32>,

    /// Whether the entity is resizable on the X axis.
    pub resizeable_x: bool,

    /// Whether the entity is resizable on the Y axis.
    pub resizeable_y: bool,

    /// Whether the entity is rotatable
    pub rotatable: bool,

    /// The interval of rotation.
    pub rotation_degrees: f32,

    /// Whether the entity can be flipped on the X axis.
    pub can_flip_x: bool,

    /// Whether the entity can be flipped on the Y axis.
    pub can_flip_y: bool,

    /// Whether the entity's color can be set.
    pub can_set_color: bool,

    /// Whether the entity has nodes.
    pub has_nodes: bool,

    /// The maximum number of nodes. 0 to ignore.
    pub node_limit: i32,

    /// Whether to display nodes.
    pub node_display: i32,

    /// Whether to display ghosts.
    pub node_ghost: bool,

    /// The entity's tags.
    pub tags: Vec<String>,

    /// The entity's custom values.
    pub values: Vec<ValueTemplate>,

    /// The entity's texture.
    pub texture: Option<String>,

    /// The entity's texture, encoded in base 64.
    pub texture_image: Option<String>,
}

/// An entity's shape.
#[derive(Clone, Debug, Deserialize)]
pub struct Shape {
    /// The shape's label.
    pub label: String,

    /// The points that make up the shape.
    pub points: Vec<Vec2<f32>>,
}

/// A tileset.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tileset {
    /// The name of the tileset.
    pub label: String,

    /// The path to the tileset's image, relative to the project's path.
    pub path: PathBuf,

    /// The tileset's image, encoded in base 64.
    pub image: String,

    /// The width of each tile in the tileset.
    pub tile_width: i32,

    /// The height of each tile in the tileset.
    pub tile_height: i32,

    /// The number of empty pixels that seperate each tile on the X axis in the tileset.
    pub tile_separation_x: i32,

    /// The number of empty pixels that seperate each tile on the Y axis in the tileset.
    pub tile_separation_y: i32,
}

impl Tileset {
    /// Returns an iterator which yields the position of each tile in the tileset.
    ///
    /// As the Ogmo project doesn't store the width and height of the texture (only the
    /// path to it), you must provide these values yourself.
    pub fn tile_coords(
        &self,
        texture_width: i32,
        texture_height: i32,
    ) -> impl Iterator<Item = Vec2<i32>> + '_ {
        let step_x = self.tile_width + self.tile_separation_x;
        let step_y = self.tile_height + self.tile_separation_y;

        let tiles_x = texture_width / step_x;
        let tiles_y = texture_height / step_y;

        (0..tiles_y).flat_map(move |tile_y| {
            (0..tiles_x).map(move |tile_x| {
                let x = tile_x * step_x;
                let y = tile_y * step_y;

                Vec2 { x, y }
            })
        })
    }
}
