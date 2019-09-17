use amethyst::core::{Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::dodge::{Player, Velocity};
use crate::util::{clamp, normalize};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (players, mut transforms, mut velocities, input, time): Self::SystemData) {
        for (player, transform, velocity) in (&players, &mut transforms, &mut velocities).join() {
            let x_in = input.axis_value("x_in");
            let y_in = input.axis_value("y_in");
            let [x_norm, y_norm] = normalize([x_in.unwrap_or(0.0), y_in.unwrap_or(0.0)]);

            let static_decel_x = -velocity.0 * 3.0 * time.delta_seconds();
            let static_decel_y = -velocity.1 * 3.0 * time.delta_seconds();
            let v_x_delta = x_norm * player.acceleration * time.delta_seconds() + static_decel_x;
            let v_y_delta = y_norm * player.acceleration * time.delta_seconds() + static_decel_y;

            let v_x = clamp(velocity.0 + v_x_delta, -player.terminal_velocity_x, player.terminal_velocity_x);
            let v_y = clamp(velocity.1 + v_y_delta, -player.terminal_velocity_y, player.terminal_velocity_y);

            velocity.0 = v_x;
            velocity.1 = v_y;
            if velocity.0 > 0.05 {
                transform.set_rotation_y_axis(0.0);
            } else if velocity.0 < -0.05 {
                transform.set_rotation_y_axis(std::f32::consts::PI);
            }
        }
    }
}
