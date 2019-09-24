//! This module is responsible for spawning the laser entity. A lot of it is
//! fairly standard, but many of the parameters need to be generalized or
//! accessed as some shared game state.
//!
//! In some cases, debugging values are used, and thus this code is not
//! independent of the environment in which it is typically run.

use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage};
use amethyst::renderer::palette::Srgb;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::SpriteRender;

use rand;

use crate::dodge::{Laser, Player, SpriteMap, Velocity};
use crate::util::{normalize, scale};

///
pub struct LaserSpawnerSystem {
    counter: u32,
    spawn_rate: u32,
}

impl Default for LaserSpawnerSystem {
    fn default() -> Self {
        Self {
            counter: 0,
            // TODO: This should be controlled by a configuration file for easy
            // tweaking during development.
            spawn_rate: 3,
        }
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
        WriteStorage<'s, Velocity>,
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
            mut velocities,
            mut sprite_renderers,
            mut tints,
            sprite_map,
            entities
        ): Self::SystemData
    ) {
        // Increase the spawn countdown. This should definitely not be linked to
        // the update rate ans should be tied to wall time.
        self.counter += 1;
        if self.counter > self.spawn_rate {
            // Get the player's transform.
            // TODO: is this idiomatic?
            let player_transform = (&players, &transforms)
                .join()
                .next()
                .map(|(_, t)| t) // Extract the transform.
                .unwrap();
            let (player_x, player_y) = {
                let player_translation = player_transform.translation();
                (player_translation[0], player_translation[1])
            };

            // Create the laser's transform and put it off the screen.
            // TODO: These numbers cannot be hard coded.
            let mut laser_transform = Transform::default();
            let (laser_x, laser_y) = if rand::random() {
                (rand::random::<f32>() * 1000.0, -32.0)
            } else {
                (rand::random::<f32>() * 1000.0, 1032.0)
            };
            laser_transform.set_translation_xyz(laser_x, laser_y, 0.0);

            // Calculate the angle from the player to the laser.
            // TODO: is this idiomatic? should I be using the math libraries
            // included with amethyst or is there a way to do this in the
            // transform?
            let theta = (player_x - laser_x).atan2(player_y - laser_y);
            laser_transform.prepend_rotation_z_axis(theta);

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
            };

            // Point the laser at the player and fire at a reasonable velocity.
            // TODO: some variance on the angle at which the laser is rotated
            // will likely create a more interesting experience. or,  perhaps a
            // time-synchronized velocity for groups of lasers?
            // TODO: laser speed in a config file.
            let laser_velocity = {
                let [v_x, v_y] = scale(normalize([player_x - laser_x, player_y - laser_y]), 100.0);
                Velocity(v_x, v_y)
            };

            // Construct the entity and add it to the scene.
            entities.build_entity()
                .with(sprite_renderer, &mut sprite_renderers)
                .with(laser_transform, &mut transforms)
                .with(laser_velocity, &mut velocities)
                .with(laser, &mut lasers)
                .with(Tint(Srgb::from(laser_color).into()), &mut tints)
                .build();

            self.counter = 0;
        }
    }
}
