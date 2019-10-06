//! This module is responsible for checking for the collision of lasers. It may
//! be generalized for collisions between any object in some future iteration
//! of this code.
use std::path::Path;

use amethyst::config::{Config, ConfigError};
use amethyst::ecs::{Entities, Join, ReadStorage, System,};

use serde::{Deserialize, Serialize};

use crate::collisions::box_collision::*;
use crate::components::{BoundingBox, Laser, Player, RelativeLocomotor};
use crate::vector::StorageTy;

// TODO: hopefully remove the `Default` derivation pending this issue:
// https://github.com/amethyst/amethyst/issues/1954
#[derive(Default, Deserialize, Serialize)]
pub struct LaserCollisionConfig {
    despawn_dist: StorageTy,
}

pub struct LaserCollisionSystem {
    despawn_dist_squared: StorageTy,
}

impl LaserCollisionSystem {
    /// Builds a `LaserCollisionSystem` with the provided `LaserCollisionConfig`.
    pub fn from_config(config: impl Into<LaserCollisionConfig>) -> Self {
        let config = config.into();
        let despawn_dist_squared = config.despawn_dist * config.despawn_dist;
        Self {
            despawn_dist_squared,
        }
    }

    /// Builds a `LaserCollisionSystem` by reading the RON file at `path`.
    pub fn from_config_path(
        path: impl AsRef<Path>
    ) -> Result<Self, ConfigError> {
        // TODO: hopefully change this to just call load pending this issue:
        // https://github.com/amethyst/amethyst/issues/1954
        LaserCollisionConfig::load_no_fallback(path).map(Self::from_config)
    }
}

impl<'s> System<'s> for LaserCollisionSystem {
    // TODO: is this idiomatic? Can I package these in some more convenient
    // structure?
    type SystemData = (
        ReadStorage<'s, Laser>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, BoundingBox>,
        ReadStorage<'s, RelativeLocomotor>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        // TODO: is this idiomatic? Can I package these in some more convenient
        // structure?
        (
            lasers,
            players,
            bounding_boxes,
            locomotors,
            entities
        ): Self::SystemData
    ) {
        // Get the player's bounding box and locomotor.
        // TODO: is this idiomatic?
        let (bb, player_locomotor) = (&players, &bounding_boxes, &locomotors)
            .join()
            .next()
            .map(|(_, &bb, l)| (bb, l))
            // This unwrap shouldn't fail since we should always have a player
            // with these components. If there is some way to get the singleton
            // player more conveniently, this will go away.
            .unwrap();

        let player_pos = player_locomotor.pos;
        let player_radius = bb.dist();

        // Calculate the player's actual bounding box as a `Rect`.
        let player_rect = Rect::from_bounding_box(player_pos, bb);

        // The iterator over all laser entities. We include `entities` in our
        // join because we need a reference to the actual entity to remove it
        // if a collision has occurred.
        let laser_iter = (&entities, &lasers, &locomotors).join();
        for (entity, laser, locomotor) in laser_iter {
            let laser_pos = locomotor.pos;
            let dist_vec = player_pos.coords - laser_pos.coords;
            let dist_squared = dist_vec.norm_squared();
            let player_radius_with_laser = laser.len + player_radius;
            let player_radius_with_laser_squared =
                player_radius_with_laser * player_radius_with_laser;
            // We are close enough to check for a collision.
            if dist_squared < player_radius_with_laser_squared {
                let laser_dir = locomotor.velocity.normalize();

                let half_laser_vec = (laser.len * 0.5) * laser_dir;

                let laser_top = laser_pos + half_laser_vec;
                let laser_bot = laser_pos - half_laser_vec;

                if line_intersects_rect(laser_top, laser_bot, player_rect) {
                    // Panic in case of entity deletion failure.
                    entities.delete(entity)
                        .expect("The deletion of an entity failed?");
                }
            } else if dist_squared > self.despawn_dist_squared {
                // Panic in case of entity deletion failure.
                entities.delete(entity)
                    .expect("The deletion of an entity failed?");
            }
        }
    }
}
