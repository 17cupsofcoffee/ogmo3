//! This example shows how data can be extracted from an Ogmo project and level, and converted into a format
//! suitable for rendering.
//!
//! The below code should not be taken as the 'one true way' of handling level data - it is recommended that
//! you use it as a reference so that you can implement a runtime format that is more tailored to your
//! particular game or engine. For example, if you know you're never going to use the 2D storage
//! variants of layers, there's no reason to write code to parse them!
//!
//! This code is adapted from https://github.com/Ogmo-Editor-3/ogmo-3-lib/blob/master/sample/Main.hx.

use std::path::PathBuf;

use hashbrown::HashMap;
use ogmo3::level::LayerData;
use ogmo3::{Level, Project};
use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

struct GameState {
    color_texture: Texture,
    tilesets: Vec<TilesetData>,
    decals: Vec<Texture>,
    sprites: Vec<Sprite>,
}

impl GameState {
    fn new(ctx: &mut Context) -> anyhow::Result<GameState> {
        // All paths in Ogmo projects and levels are relative to the project folder.

        let base_path = PathBuf::from("./examples/sample_project");
        let project = Project::from_file(base_path.join("test.ogmo"))?;
        let level = Level::from_file(base_path.join("levels/uno.json"))?;

        // Most of the project file's data can be ignored at runtime, but it does
        // provide info which can be used to slice up tilesets.
        //
        // In this example, we store the tileset data in a Vec, and convert the
        // string IDs to indices for quick lookup.

        let mut tilesets = Vec::new();
        let mut tileset_mappings = HashMap::new();

        for tileset in project.tilesets {
            let texture = Texture::new(ctx, base_path.join(&tileset.path))?;

            let tiles = tileset
                .tile_coords(texture.width(), texture.height())
                .map(|t| {
                    Rectangle::new(
                        t.x as f32,
                        t.y as f32,
                        tileset.tile_width as f32,
                        tileset.tile_height as f32,
                    )
                })
                .collect();

            let id = tilesets.len();
            tilesets.push(TilesetData { texture, tiles });
            tileset_mappings.insert(tileset.label, id);
        }

        // In this example, we convert the layers into a single flat list of sprite data, which we can
        // then iterate over in order to render the level. You may want to take a different approach
        // in your game - this crate does not enforce any particular runtime format.
        //
        // We also parse the decals from the level data, rather than the project data, as that way
        // we can only load what is used rather than having to load the entire folder. Like the
        // tilesets, we'll store them using indexes for quick lookups.

        let mut sprites = Vec::new();
        let mut decals = Vec::new();

        // TODO: Ogmo allows you to specify layer offsets, which can be useful for creating
        // chunked levels - this example does not currently take those fields into account.

        for layer in level.layers {
            match layer.data {
                LayerData::Tile(data) => {
                    // In tile layers, each tile is represented by a number. This can be used as an
                    // index into our sliced tileset from earlier.
                    //
                    // If the value is -1, it means the tile is empty.

                    for (i, &t) in data.data.iter().enumerate() {
                        if t > -1 {
                            let px = (i as i32 % layer.grid_cells_x) * layer.grid_cell_width;
                            let py = (i as i32 / layer.grid_cells_x) * layer.grid_cell_height;

                            sprites.push(Sprite::TileIndex {
                                tileset: tileset_mappings[&data.tileset],
                                tile: t as usize,
                                position: Vec2::new(px as f32, py as f32),
                            });
                        }
                    }
                }

                LayerData::Tile2D(data) => {
                    // 2D tile layers use the same kind of values as 1D tile layers - see above.

                    for (y, row) in data.data_2d.iter().enumerate() {
                        for (x, &t) in row.iter().enumerate() {
                            if t > -1 {
                                let px = x as i32 * layer.grid_cell_width;
                                let py = y as i32 * layer.grid_cell_height;

                                sprites.push(Sprite::TileIndex {
                                    tileset: tileset_mappings[&data.tileset],
                                    tile: t as usize,
                                    position: Vec2::new(px as f32, py as f32),
                                });
                            }
                        }
                    }
                }

                LayerData::TileCoords(data) => {
                    // In tile co-ord layers, each tile is represented by a two-element array,
                    // which contains the co-ordinates of the tile in the tileset.
                    //
                    // If the value is [-1], it means the tile is empty.

                    for (i, coords) in data.data_coords.iter().enumerate() {
                        if coords[0] > -1 {
                            let px = (i as i32 % layer.grid_cells_x) * layer.grid_cell_width;
                            let py = (i as i32 / layer.grid_cells_x) * layer.grid_cell_height;
                            let pu = coords[0] * layer.grid_cell_width;
                            let pv = coords[1] * layer.grid_cell_height;

                            sprites.push(Sprite::TileUV {
                                tileset: tileset_mappings[&data.tileset],
                                uv: Rectangle::new(
                                    pu as f32,
                                    pv as f32,
                                    layer.grid_cell_width as f32,
                                    layer.grid_cell_height as f32,
                                ),
                                position: Vec2::new(px as f32, py as f32),
                            });
                        }
                    }
                }

                LayerData::TileCoords2D(data) => {
                    // 2D tile co-ord layers use the same kind of values as 1D tile co-ord
                    // layers - see above.

                    for (y, row) in data.data_coords_2d.iter().enumerate() {
                        for (x, coords) in row.iter().enumerate() {
                            if coords[0] > -1 {
                                let px = x as i32 * layer.grid_cell_width;
                                let py = y as i32 * layer.grid_cell_height;
                                let pu = coords[0] * layer.grid_cell_width;
                                let pv = coords[1] * layer.grid_cell_height;

                                sprites.push(Sprite::TileUV {
                                    tileset: tileset_mappings[&data.tileset],
                                    uv: Rectangle::new(
                                        pu as f32,
                                        pv as f32,
                                        layer.grid_cell_width as f32,
                                        layer.grid_cell_height as f32,
                                    ),
                                    position: Vec2::new(px as f32, py as f32),
                                });
                            }
                        }
                    }
                }

                LayerData::Grid(data) => {
                    // In grid layers, each cell contains a string, chosen from a set of values
                    // defined in the layer config.
                    //
                    // By default, "0" is used as the empty value, but this can be changed in
                    // the editor per-layer.

                    for (i, v) in data.grid.iter().enumerate() {
                        if v != "0" {
                            let px = (i as i32 % layer.grid_cells_x) * layer.grid_cell_width;
                            let py = (i as i32 / layer.grid_cells_x) * layer.grid_cell_height;

                            sprites.push(Sprite::Rect {
                                rect: Rectangle::new(
                                    px as f32,
                                    py as f32,
                                    layer.grid_cell_width as f32,
                                    layer.grid_cell_height as f32,
                                ),
                                color: Color::BLACK,
                            });
                        }
                    }
                }

                LayerData::Grid2D(data) => {
                    // 2D grid layers use the same kind of values as 1D grid layers - see above.

                    for (y, row) in data.grid_2d.iter().enumerate() {
                        for (x, v) in row.iter().enumerate() {
                            if v != "0" {
                                let px = x as i32 * layer.grid_cell_width;
                                let py = y as i32 * layer.grid_cell_height;

                                sprites.push(Sprite::Rect {
                                    rect: Rectangle::new(
                                        px as f32,
                                        py as f32,
                                        layer.grid_cell_width as f32,
                                        layer.grid_cell_height as f32,
                                    ),
                                    color: Color::BLACK,
                                });
                            }
                        }
                    }
                }

                LayerData::Entity(data) => {
                    // In a real game, you would probably not want to draw entity data directly like
                    // this - instead, you would use them to spawn game entities at the specified
                    // location.

                    for entity in &data.entities {
                        sprites.push(Sprite::Rect {
                            color: Color::RED,
                            rect: Rectangle::new(entity.x, entity.y, 16.0, 16.0),
                        });
                    }
                }

                LayerData::Decal(data) => {
                    // Decal data is stored as a path (relative to the layer's defined folder) and
                    // positioning data.
                    //
                    // In this example, we load a seperate texture for every decal - in a real game,
                    // you would probably want to make sure you don't load the same texture multiple
                    // times!

                    let folder_path = base_path.join(data.folder);

                    for decal in data.decals {
                        let texture = Texture::new(ctx, folder_path.join(decal.texture))?;
                        let id = decals.len();

                        decals.push(texture);
                        sprites.push(Sprite::Decal {
                            decal: id,
                            position: Vec2::new(decal.x, decal.y),
                            rotation: decal.rotation.unwrap_or(0.0),
                            scale: Vec2::new(
                                decal.scale_x.unwrap_or(1.0),
                                decal.scale_y.unwrap_or(1.0),
                            ),
                        });
                    }
                }
            }
        }

        Ok(GameState {
            color_texture: Texture::from_rgba(ctx, 1, 1, &[255, 255, 255, 255])?,
            tilesets,
            decals,
            sprites,
        })
    }
}

impl State<anyhow::Error> for GameState {
    fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        graphics::clear(ctx, Color::WHITE);

        for sprite in &self.sprites {
            match sprite {
                Sprite::TileIndex {
                    tileset,
                    tile,
                    position,
                } => {
                    let tileset = &self.tilesets[*tileset];

                    graphics::draw(
                        ctx,
                        &tileset.texture,
                        DrawParams::new()
                            .position(*position)
                            .clip(tileset.tiles[*tile]),
                    );
                }

                Sprite::TileUV {
                    tileset,
                    uv,
                    position,
                } => {
                    let tileset = &self.tilesets[*tileset];

                    graphics::draw(
                        ctx,
                        &tileset.texture,
                        DrawParams::new().position(*position).clip(*uv),
                    );
                }

                Sprite::Rect { rect, color } => {
                    graphics::draw(
                        ctx,
                        &self.color_texture,
                        DrawParams::new()
                            .position(Vec2::new(rect.x, rect.y))
                            .scale(Vec2::new(rect.width, rect.height))
                            .color(*color),
                    );
                }

                Sprite::Decal {
                    decal,
                    position,
                    rotation,
                    scale,
                } => {
                    let texture = &self.decals[*decal];

                    graphics::draw(
                        ctx,
                        texture,
                        DrawParams::new()
                            .position(*position)
                            .rotation(*rotation)
                            .scale(*scale),
                    );
                }
            }
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    ContextBuilder::new("Rendering an Ogmo Project", 640, 480)
        .build()?
        .run(GameState::new)
}

enum Sprite {
    TileIndex {
        tileset: usize,
        tile: usize,
        position: Vec2<f32>,
    },
    TileUV {
        tileset: usize,
        uv: Rectangle,
        position: Vec2<f32>,
    },
    Rect {
        rect: Rectangle,
        color: Color,
    },
    Decal {
        decal: usize,
        position: Vec2<f32>,
        rotation: f32,
        scale: Vec2<f32>,
    },
}

struct TilesetData {
    texture: Texture,
    tiles: Vec<Rectangle>,
}
