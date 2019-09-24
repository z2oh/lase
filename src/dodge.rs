//! This module holds the core gameplay infrastructure code.
//!
//! Currently, this module also serves as a testing bed for new ideas. Much of
//! the specialized gameplay code in this module will be grouped and factored
//! into other compilation units, once it has demonstrated its sustained
//! usefulness.

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        Camera,
        ImageFormat,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
    },
    window::{ScreenDimensions},
};
use std::collections::HashMap;

/// The main gameplay state.
#[derive(Default)]
pub struct Dodge;

impl SimpleState for Dodge {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // Initialize global resources.
        let sprite_map = load_sprite_map(data.world);
        data.world.add_resource(sprite_map);

        let time_scale = TimeScale::default();
        data.world.add_resource(time_scale);

        // Initialize singleton entities.
        initialize_player(data.world);
        initialize_camera(data.world);
    }
}

// TODO: Factor these structs into their own module(s).

//
// Resources.
//

/// Holds a hash map mapping string ids to a sprite sheet handle. This allows
/// easy look up of sprites in systems.
// TODO: Make this type checked by having some kind of enum system for hardcoded
// texture ids?
// TODO: Generalize this to work with generic asset types.
#[derive(Default)]
pub struct SpriteMap(HashMap<String, Handle<SpriteSheet>>);
// TODO: Evaluate this public API.
impl SpriteMap {
    /// Inserts a new handle with its string id into the map.
    #[allow(dead_code)]
    pub fn insert(
        &mut self,
        k: String,
        v: Handle<SpriteSheet>
    ) -> Option<Handle<SpriteSheet>> {
        self.0.insert(k, v)
    }

    /// Gets the handle to the sprite sheet. Since `Handle` clones are cheap, we
    /// clone here so that the caller doesn't have to.
    pub fn get(&self, k: &str) -> Option<Handle<SpriteSheet>> {
        self.0.get(k).map(Clone::clone)
    }
}

/// Wrapper around a float that controls how fast time within the game is
/// moving. This is a multiplicative factor.
pub struct TimeScale(pub f32);
impl Default for TimeScale {
    /// The default timescale is 1.0, so time will move at the standard speed.
    fn default() -> Self {
        Self(1.0)
    }
}

//
// Components.
//

/// This component holds properties about a laser.
pub struct Laser {
    // TODO: is this idiomatic? I should probably be using amethyst's color
    // types.
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
    // TODO: investigate storage types. This component in particular should
    // probably use `VecStorage`.
    type Storage = DenseVecStorage<Self>;
}

/// This component holds properties about a player. The current fields of this
/// struct will probably be factored into components of their own.
// TODO: is this idiomatic for singleton entities like the player?
pub struct Player {
    pub terminal_velocity_x: f32,
    pub terminal_velocity_y: f32,
    pub acceleration: f32,
}

impl Component for Player {
    // TODO: investigate storage types.
    type Storage = DenseVecStorage<Self>;
}

/// This component holds velocity information about an entity in the form of a
/// two dimensional vector. This information encodes both speed and direction.
pub struct Velocity(pub f32, pub f32);

impl Component for Velocity {
    // TODO: investigate storage types. This component in particular should
    // probably use `VecStorage`.
    type Storage = DenseVecStorage<Self>;
}

/// This is the width and height of the bounding box. We only permit this
/// component to be added to an entity with a transform, so that the box has a
/// position.
// TODO: group this in some sort of "collidable" component?
// TODO: is this idiomatic?
pub struct BoundingBox(pub f32, pub f32);

impl Component for BoundingBox {
    // TODO: investigate storage types.
    type Storage = DenseVecStorage<Self>;
}

/// Static function to initialize a player in a world.
fn initialize_player(world: &mut World) {
    // TODO: the screen dimensions should be abstracted from the world's
    // coordinates.
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    // Read the player's sprite sheet.
    let sprite_sheet = {
        let sprite_map = world.read_resource::<SpriteMap>();
        // TODO: Make this type checked by having some kind of enum system for
        // hardcoded texture ids?
        sprite_map.get("player").unwrap()
    };

    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };

    // Set the player's position to the middle of the world.
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(width * 0.5, height * 0.5, 0.0);

    // Create the player entity with lots of hard coded values.
    world.create_entity()
        .with(sprite_render)
        .with(local_transform)
        .with(Velocity(0.0, 0.0))
        .with(Player {
            terminal_velocity_x: 100.0,
            terminal_velocity_y: 100.0,
            acceleration: 200.0,
        })
        .with(BoundingBox(8.0, 8.0))
        .build();
}

/// Static function to initialize a camera in a world.
fn initialize_camera(world: &mut World) {
    // TODO: the screen dimensions should be abstracted from the world's
    // coordinates.
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    // Create the camera entity.
    world
        .create_entity()
        // TODO: evaluate these parameters.
        .with(Camera::standard_2d(width / 4.0, height / 4.0))
        .with(transform)
        .build();
}

/// Static function to construct the initial sprite map for the world.
/// _N.B.: this currently hard codes the paths to resources._
// TODO: load assets from some sort of manifest file.
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
        // TODO: Make this type checked by having some kind of enum system for
        // hardcoded texture ids?
        ("laser_sprite".to_string(), laser_sprite_sheet_handle),
        ("player".to_string(), sprites_sprite_sheet_handle),
    ].iter().cloned().collect())
}
