extern crate amethyst;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
    Texture, TextureMetadata,
};

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {}
}
