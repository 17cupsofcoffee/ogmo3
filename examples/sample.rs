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
use ogmo3::{Layer, Level, Project};
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
            match layer {
                // Ogmo's tile data can be quite involved to unpack, and there are multiple different
                // storage options available in the editor. The `unpack` method abstracts over these,
                // allowing you to quickly pull tile data out of the layer.
                Layer::Tile(layer) => {
                    for tile in layer.unpack() {
                        if let Some(id) = tile.id {
                            sprites.push(Sprite::TileIndex {
                                tileset: tileset_mappings[&layer.tileset],
                                tile: id as usize,
                                position: Vec2::new(
                                    tile.pixel_position.x as f32,
                                    tile.pixel_position.y as f32,
                                ),
                            });
                        }
                    }
                }

                // An `unpack` method is also available for layers defined using tile co-ordinates.
                Layer::TileCoords(layer) => {
                    for tile in layer.unpack() {
                        if let Some(coords) = tile.pixel_coords {
                            sprites.push(Sprite::TileUV {
                                tileset: tileset_mappings[&layer.tileset],
                                uv: Rectangle::new(
                                    coords.x as f32,
                                    coords.y as f32,
                                    layer.grid_cell_width as f32,
                                    layer.grid_cell_height as f32,
                                ),
                                position: Vec2::new(
                                    tile.pixel_position.x as f32,
                                    tile.pixel_position.y as f32,
                                ),
                            });
                        }
                    }
                }

                // An `unpack` method is also available for grid data.
                Layer::Grid(layer) => {
                    for cell in layer.unpack() {
                        if cell.value != "0" {
                            sprites.push(Sprite::Rect {
                                rect: Rectangle::new(
                                    cell.pixel_position.x as f32,
                                    cell.pixel_position.y as f32,
                                    layer.grid_cell_width as f32,
                                    layer.grid_cell_height as f32,
                                ),
                                color: Color::BLACK,
                            });
                        }
                    }
                }

                // In a real game, you would probably not want to draw entity data directly like
                // this - instead, you would use them to spawn game entities at the specified
                // location.
                Layer::Entity(layer) => {
                    for entity in &layer.entities {
                        sprites.push(Sprite::Rect {
                            color: Color::RED,
                            rect: Rectangle::new(entity.x, entity.y, 16.0, 16.0),
                        });
                    }
                }

                // Decal data is stored as a path (relative to the layer's defined folder) and
                // positioning data.
                //
                // In this example, we load a seperate texture for every decal - in a real game,
                // you would probably want to make sure you don't load the same texture multiple
                // times!
                Layer::Decal(layer) => {
                    let folder_path = base_path.join(layer.folder);

                    for decal in layer.decals {
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
                    let uv = tileset.tiles[*tile];

                    tileset.texture.draw_region(ctx, uv, *position);
                }

                Sprite::TileUV {
                    tileset,
                    uv,
                    position,
                } => {
                    let tileset = &self.tilesets[*tileset];

                    tileset.texture.draw_region(ctx, *uv, *position);
                }

                Sprite::Rect { rect, color } => {
                    self.color_texture.draw(
                        ctx,
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

                    texture.draw(
                        ctx,
                        DrawParams::new()
                            .origin(Vec2::new(
                                texture.width() as f32 / 2.0,
                                texture.height() as f32 / 2.0,
                            ))
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
