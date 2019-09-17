use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::{ScreenDimensions},
};
use std::collections::HashMap;

#[derive(Default)]
pub struct Dodge;

impl SimpleState for Dodge {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let sprite_map = load_sprite_map(data.world);

        data.world.add_resource(sprite_map);

        initialize_player(data.world);
        initialize_camera(data.world);
    }
}

#[derive(Default)]
pub struct SpriteMap(HashMap<String, Handle<SpriteSheet>>);
impl SpriteMap {
    #[allow(dead_code)]
    pub fn insert(&mut self, k: String, v: Handle<SpriteSheet>) -> Option<Handle<SpriteSheet>> {
        self.0.insert(k, v)
    }

    pub fn get(&self, k: &str) -> Option<Handle<SpriteSheet>> {
        self.0.get(k).map(Clone::clone)
    }
}

pub struct Laser {
    pub color: (f32, f32, f32)
}

impl Default for Laser {
    fn default() -> Self {
        Self {
            color: (0.0, 0.0, 0.0),
        }
    }
}

impl Component for Laser {
    type Storage = DenseVecStorage<Self>;
}

pub struct Player {
    pub terminal_velocity_x: f32,
    pub terminal_velocity_y: f32,
    pub acceleration: f32,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub struct Velocity(pub f32, pub f32);

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_player(world: &mut World) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let sprite_sheet = {
        let sprite_map = world.read_resource::<SpriteMap>();
        sprite_map.get("player").unwrap()
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(width / 2.0, height / 2.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 0,
    };

    world.create_entity()
        .with(sprite_render)
        .with(local_transform)
        .with(Velocity(0.0, 0.0))
        .with(Player {
            terminal_velocity_x: 100.0,
            terminal_velocity_y: 100.0,
            acceleration: 200.0,
        })
        .build();
}

fn initialize_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width / 4.0, height / 4.0))
        .with(transform)
        .build();
}

fn load_sprite_map(world: &mut World) -> SpriteMap {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let laser_texture_handle = loader.load(
        "texture/laser.png",
        ImageFormat::default(),
        (),
        &texture_storage,
    );
    let sprites_texture_handle = loader.load(
        "texture/player.png",
        ImageFormat::default(),
        (),
        &texture_storage,
    );

    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    let laser_sprite_sheet_handle = loader.load(
        "texture/laser.ron",
        SpriteSheetFormat(laser_texture_handle),
        (),
        &sprite_sheet_storage,
    );
    let sprites_sprite_sheet_handle = loader.load(
        "texture/player.ron",
        SpriteSheetFormat(sprites_texture_handle),
        (),
        &sprite_sheet_storage,
    );

    SpriteMap([
        ("laser_sprite".to_string(), laser_sprite_sheet_handle),
        ("player".to_string(), sprites_sprite_sheet_handle),
    ].iter().cloned().collect())
}
