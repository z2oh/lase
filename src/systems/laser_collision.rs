//! This module is responsible for checking for the collision of lasers. It may
//! be generalized for collisions between any object in some future iteration
//! of this code.

// TODO: urgently need some way to clean up lasers after they have left (i.e.
// "collided with") the border of the screen. The only open question here is how
// to check for collisions in one direction.
// IDEA: count for multiple collisions, each ticking a flag? requires extended
// knowledge of collision (`is_colliding` flag for example).
// IDEA: collide with only certain sides of a box collider. this can be
// incorporated with bitflags crate to make a very efficient check against an
// enclosing box collider (i.e. the entire arena).

use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, ReadStorage, System,};

use crate::dodge::{BoundingBox, Laser, Player};
use crate::util::line_intersects_rect;

pub struct LaserCollisionSystem;

impl<'s> System<'s> for LaserCollisionSystem {
    // TODO: is this idiomatic? Can I package these in some more convenient
    // structure?
    type SystemData = (
        ReadStorage<'s, Laser>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, BoundingBox>,
        ReadStorage<'s, Transform>,
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
            transforms,
            entities
        ): Self::SystemData
    ) {
        // Get the player's bounding box and transform.
        // TODO: is this idiomatic?
        let (bb, player_transform) = (&players, &bounding_boxes, &transforms)
            .join()
            .next()
            .map(|(_, bb, t)| (bb, t))
            // This unwrap shouldn't fail since we should always have a player
            // with these components. If there is some way to get the singleton
            // player more conveniently, this will go away.
            .unwrap();

        // Get the player's current position.
        let (player_x, player_y) = {
            let player_translation = player_transform.translation();
            (player_translation[0], player_translation[1])
        };

        // Calculate the player's actual bounding box in a form that our
        // collision utility functions can accept.
        let player_rect = (
            player_x - bb.0 / 2.0,
            player_y - bb.1 / 2.0,
            bb.0,
            bb.1
        );

        // The iterator over all laser entities. We include `entities` in our
        // join because we need a reference to the actual entity to remove it
        // if a collision has occurred.
        let laser_iter = (&entities, &lasers, &transforms).join();
        for (entity, _, transform) in laser_iter {
            let (laser_x, laser_y) = {
                let laser_translation = transform.translation();
                (laser_translation[0], laser_translation[1])
            };

            // Calculate the angle at which the laser is pointing.
            // TODO: is this idiomatic?
            let laser_raw_angle = transform.rotation().euler_angles().2;

            // The raw angle is perpendicular to the actual angle of travel for
            // the laser. We account for that here by adding pi/2.
            let angle = laser_raw_angle + std::f32::consts::PI / 2.0;

            // TODO: remove hard-coded 16.0 length of laser. This should most
            // likely be a property of the laser.
            let laser_x_offset = angle.cos() * 16.0;
            let laser_y_offset = angle.sin() * 16.0;

            // Calculate the points of the line segment representing the laser.
            let laser_top = (
                laser_x - laser_x_offset,
                laser_y - laser_y_offset,
            );
            let laser_bot = (
                laser_x + laser_x_offset,
                laser_y + laser_y_offset,
            );

            // If the laser collides with the player, we delete it. This will
            // of course become more complex when additional gameplay code is in
            // place.
            if line_intersects_rect(laser_top, laser_bot, player_rect) {
                // Panic in case of entity deletion failure.
                entities.delete(entity)
                    .expect("The deletion of an entity failed?");
            }
        }
    }
}
