//! This module holds the core gameplay infrastructure code.
//!
//! Currently, this module also serves as a testing bed for new ideas. Much of
//! the specialized gameplay code in this module will be grouped and factored
//! into other compilation units, once it has demonstrated its sustained
//! usefulness.
use std::path::{Path, PathBuf};

use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{
        Camera,
        ImageFormat,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
    },
    utils::ortho_camera::{CameraNormalizeMode, CameraOrtho, CameraOrthoWorldCoordinates},
    window::{ScreenDimensions},
};

use crate::resources::{SpriteMap, TimeScale};
use crate::components::{BoundingBox, Player, RelativeLocomotor};
use crate::vector::prelude::*;

/// The main gameplay state.
pub struct Dodge {
    config_path: PathBuf,
}

impl Dodge {
    pub fn with_config_path(config_path: PathBuf) -> Self {
        Self {
            config_path
        }
    }
}

impl SimpleState for Dodge {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // Initialize global resources.
        let sprite_map = load_sprite_map(data.world);
        data.world.insert(sprite_map);

        let time_scale = TimeScale::default();
        data.world.insert(time_scale);

        // Initialize singleton entities.
        initialize_player(data.world, self.config_path.join("player.ron"));
        initialize_camera(data.world);
    }
}

/// Static function to initialize a player in a world.
fn initialize_player(world: &mut World, config_path: impl AsRef<Path>) {
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
    let player_origin = Point2::new(width * 0.5, height * 0.5);
    let local_transform = Transform::from(add_dim(player_origin.coords));

    // Create the player entity with lots of hard coded values.
    world.create_entity()
        .with(sprite_render)
        .with(local_transform)
        .with(RelativeLocomotor::with_pos(player_origin))
        // Explicit panic if an error is encountered while reading the config
        // file.
        .with(Player::from_config_path(config_path).unwrap())
        .with(BoundingBox::from(Vec2::new(4.0, 4.0)))
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

    let transform = Transform::from(Vec3::new(width * 0.5, height * 0.5, 10.0));

    // Create the camera entity.
    world
        .create_entity()
        .with(transform)
        // Upscale rendering by 4x.
        .with(Camera::standard_2d(width * 0.5 / 4.0, height * 0.5 / 4.0))
        // The CameraOrtho component will ensure that the world coordinates
        // specified by the CameraOrthoWorldCoordinates will remain visible
        // through window resizes. If the aspect ratio of the window does not
        // match the aspect ratio of our visible world coordinates, we render
        // additional world space. This means that in extreme situations (i.e.
        // crazy window aspect ratio), we may render parts of the world very
        // very far away from the player.
        //
        // To make this more robust, we probably want to place some restrictions
        // on the resizability of the window, or possibly introduce some kind of
        // letterboxing.
        .with(CameraOrtho::new(
            CameraNormalizeMode::Contain,
            // Upscale rendering by 4x.
            CameraOrthoWorldCoordinates {
                left: -width * 0.5 / 4.0,
                right: width * 0.5 / 4.0,
                bottom: -height * 0.5 / 4.0,
                top: height * 0.5 / 4.0,
            }
        ))
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

    [
        // TODO: Make this type checked by having some kind of enum system for
        // hardcoded texture ids?
        ("laser_sprite", laser_sprite_sheet_handle),
        ("player", sprites_sprite_sheet_handle),
    ].iter().cloned().collect()
}
