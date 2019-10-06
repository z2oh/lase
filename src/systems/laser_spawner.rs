//! This module is responsible for spawning the laser entity. A lot of it is
//! fairly standard, but many of the parameters need to be generalized or
//! accessed as some shared game state.
//!
//! In some cases, debugging values are used, and thus this code is not
//! independent of the environment in which it is typically run.
use std::path::Path;

use amethyst::core::Transform;
use amethyst::config::{Config, ConfigError};
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage};
use amethyst::renderer::palette::Srgb;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::SpriteRender;

use serde::{Deserialize, Serialize};

use crate::components::{Laser, Player, RelativeLocomotor};
use crate::resources::SpriteMap;
use crate::vector::prelude::*;

// TODO: hopefully remove the `Default` derivation pending this issue:
// https://github.com/amethyst/amethyst/issues/1954
#[derive(Default, Deserialize, Serialize)]
pub struct LaserSpawnerConfig {
    spawn_rate: u32,
    spawn_dist: StorageTy,
}

pub struct LaserSpawnerSystem {
    counter: u32,
    config: LaserSpawnerConfig,
}

impl LaserSpawnerSystem {
    /// Builds a `LaserSpawnerSystem` with the provided `LaserSpawnerConfig`.
    pub fn from_config(config: impl Into<LaserSpawnerConfig>) -> Self {
        Self {
            counter: 0,
            config: config.into(),
        }
    }

    /// Builds a `LaserSpawnerSystem` by reading the RON file at `path`.
    pub fn from_config_path(
        path: impl AsRef<Path>
    ) -> Result<Self, ConfigError> {
        // TODO: hopefully change this to just call load pending this issue:
        // https://github.com/amethyst/amethyst/issues/1954
        LaserSpawnerConfig::load_no_fallback(path).map(Self::from_config)
    }
}

impl<'s> System<'s> for LaserSpawnerSystem {
    // TODO: is this idiomatic? Can I package these in some more convenient
    // structure?
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Laser>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, RelativeLocomotor>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Tint>,
        Read<'s, SpriteMap>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        // TODO: is this idiomatic? Can I package these in some more convenient
        // structure?
        (
            players,
            mut lasers,
            mut transforms,
            mut locomotors,
            mut sprite_renderers,
            mut tints,
            sprite_map,
            entities,
        ): Self::SystemData
    ) {
        // Increase the spawn countdown. This should definitely not be linked to
        // the update rate ans should be tied to wall time.
        self.counter += 1;
        if self.counter > self.config.spawn_rate {
            // Get the player's locomotor.
            // TODO: is this idiomatic?
            let player_locomotor = (&players, &locomotors)
                .join()
                .next()
                .map(|(_, l)| l) // Extract the locomotor.
                .unwrap();

            // The desired spawn distance from the player.
            // TODO: make this configurable.
            let player_dist = 250.0;

            // Pick a random rotation.
            let rand_theta = rand::random::<StorageTy>() * PI * 2.0;

            // Rotate the y unit vector by our angle.
            let rotation = Rot2::new(rand_theta);
            let rotated_vec = rotation * Vec2::y();

            // Scale the rotation vector by the desired distance from the
            // player, then add the scaled vector to the player's position.
            let laser_pos = player_locomotor.pos + (player_dist * rotated_vec);

            // The laser translation. We take the 2D laser position vector, add
            // a dimension to it (z initialized to 0.0) and then convert it into
            // the `Translation3` type.
            let laser_translation = add_dim(laser_pos.coords).into();

            // The laser rotation. Calculated by creating a quaternion from a
            // scaled axis. This means the direction of our input vector
            // determines the axis around which the rotation will occur. We pass
            // the z-axis in order to rotate within the xy plane. We scale the
            // vector by our rotation angle, so that the magnitude of the vector
            // is equal to our rotation.
            let laser_rotation = Quaternion::from_axis_angle(
                &Vec3::z_axis(),
                rand_theta,
            );

            // The laser scale. We don't want to scale anything, so we pass a
            // `Vector3` of all ones.
            let laser_scale = Vec3::from_element(1.0);

            // The laser's actual transform.
            let laser_transform = Transform::new(
                laser_translation,
                laser_rotation,
                laser_scale,
            );

            // Extract the laser's sprite sheet from the sprite map.
            let sprite_sheet = sprite_map.get("laser_sprite").unwrap();
            let sprite_renderer = SpriteRender {
                sprite_sheet,
                sprite_number: 0,
            };

            // Determine the laser's tint color. For now, this color is chosen
            // at random.
            // TODO: Create colored textures on the fly to avoid the "tint"
            // look? Choose a more nuanced color selection scheme, perhaps a
            // color representation of some frequency of a signal (e.g. sound)?
            let laser_color = (rand::random(), rand::random(), rand::random());
            let laser = Laser {
                color: laser_color,
                // TODO: don't hardcode this.
                len: 32.0,
            };

            // Point the laser at the player and fire at a reasonable velocity.
            // TODO: some variance on the angle at which the laser is rotated
            // will likely create a more interesting experience. or, perhaps a
            // time-synchronized velocity for groups of lasers?
            // TODO: laser speed in a config file.
            let laser_velocity = -rotated_vec * 100.0;

            let laser_locomotor = RelativeLocomotor::with_velocity(laser_pos, laser_velocity);

            // Construct the entity and add it to the scene.
            entities.build_entity()
                .with(sprite_renderer, &mut sprite_renderers)
                .with(laser_transform, &mut transforms)
                .with(laser_locomotor, &mut locomotors)
                .with(laser, &mut lasers)
                .with(Tint(Srgb::from(laser_color).into()), &mut tints)
                .build();

            self.counter = 0;
        }
    }
}
