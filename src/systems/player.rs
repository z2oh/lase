//! This module updates the player's position response to input via a (very)
//! rudimentary twice differentiable physics model.

use amethyst::core::{Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{Player, RelativeLocomotor};
use crate::vector::prelude::*;

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    // TODO: is this idiomatic? Can I package these in some more convenient
    // structure?
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, RelativeLocomotor>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            players,
            mut transforms,
            mut locomotors,
            input,
            time
        ): Self::SystemData
    ) {
        let player_iter = (&players, &mut transforms, &mut locomotors).join();
        for (player, transform, locomotor) in player_iter {
            // We must borrow here, since we can not move `config` out of
            // `player`. This is fine, since all the values we are using from
            // the config are `Copy` anyway.
            let config = &player.config;

            // Grab the raw input values.
            let x_in = input.axis_value("x_in");
            let y_in = input.axis_value("y_in");

            // Unwrap them, defaulting to 0.0 if the axis is in deadzone.
            let input_vec = Vec2::new(x_in.unwrap_or(0.0), y_in.unwrap_or(0.0));

            // Normalize the input vector. If the input vector is (0.0, 0.0), we
            // skip normalization, as the magnitude of the vector is 0.0.
            let norm = if input_vec == Vec2::zeros() {
                input_vec
            } else {
                input_vec.normalize()
            };

            // Figure out the static deceleration, meaning the force in response
            // to "friction".
            let scaled_decel_factor =
                config.deceleration * time.delta_seconds();
            let static_decel = -locomotor.velocity * scaled_decel_factor;

            let scaled_accel_factor =
                config.acceleration * time.delta_seconds();
            let v_delta = norm * scaled_accel_factor + static_decel;

            // Our new velocity, before accounting for max speed.
            let new_velocity = locomotor.velocity + v_delta;

            // TODO: is this idiomatic?
            let new_velocity_norm_squared = new_velocity.norm_squared();
            let max_speed_squared = config.max_speed * config.max_speed;

            // Ensure our velocity does not exceed our max speed. If it does, we
            // clamp it down.
            let velocity =
                if new_velocity_norm_squared > max_speed_squared {
                    new_velocity.normalize() * config.max_speed
                } else if new_velocity_norm_squared < -max_speed_squared {
                    new_velocity.normalize() * -config.max_speed
                } else {
                    new_velocity
                };

            // Set the velocity to our calculated value.
            locomotor.velocity = velocity;

            // Rotate the player's sprite to look left or right once the
            // player's x-velocity passes some threshold of speed in the other
            // direction.
            if velocity[0] > 0.05 {
                transform.set_rotation_y_axis(0.0);
            } else if velocity[0] < -0.05 {
                transform.set_rotation_y_axis(std::f32::consts::PI);
            }
        }
    }
}
