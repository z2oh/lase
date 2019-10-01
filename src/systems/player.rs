//! This module updates the player's position response to input via a (very)
//! rudimentary twice differentiable physics model.

use amethyst::core::{Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{Player, Velocity};
use crate::util::{clamp, normalize};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    // TODO: is this idiomatic? Can I package these in some more convenient
    // structure?
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            players,
            mut transforms,
            mut velocities,
            input,
            time
        ): Self::SystemData
    ) {
        let player_iter = (&players, &mut transforms, &mut velocities).join();
        for (player, transform, velocity) in player_iter {
            // We must borrow here, since we can not move `config` out of
            // `player`. This is fine, since all the values we are using from
            // the config are `Copy` anyway.
            let config = &player.config;

            // Grab the raw input values.
            let x_in = input.axis_value("x_in");
            let y_in = input.axis_value("y_in");

            // Normalize them, defaulting to 0.0 if the axis is in deadzone.
            let [x_norm, y_norm] = normalize([
                x_in.unwrap_or(0.0),
                y_in.unwrap_or(0.0),
            ]);

            // Figure out the static deceleration, meaning the force in response
            // to "friction".
            let scaled_decel = config.deceleration * time.delta_seconds();
            let static_decel_x = -velocity.0 * scaled_decel;
            let static_decel_y = -velocity.1 * scaled_decel;

            // Calculate the change in player's velocity.
            let scaled_accel = config.acceleration * time.delta_seconds();
            let v_x_delta = x_norm * scaled_accel + static_decel_x;
            let v_y_delta = y_norm * scaled_accel + static_decel_y;

            // Calculate the new velocity of the player, clamping to ensure it
            // remains within the player's terminal velocity range.
            let v_x = clamp(
                velocity.0 + v_x_delta,
                -config.terminal_velocity_x,
                config.terminal_velocity_x
            );
            let v_y = clamp(
                velocity.1 + v_y_delta,
                -config.terminal_velocity_y,
                config.terminal_velocity_y
            );

            // Set the velocity to our calculated values.
            velocity.0 = v_x;
            velocity.1 = v_y;

            // Rotate the player's sprite to look left or right once the
            // player's x-velocity passes some threshold of speed in the other
            // direction.
            if velocity.0 > 0.05 {
                transform.set_rotation_y_axis(0.0);
            } else if velocity.0 < -0.05 {
                transform.set_rotation_y_axis(std::f32::consts::PI);
            }
        }
    }
}
