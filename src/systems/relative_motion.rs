use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::dodge::Velocity;

pub struct RelativeMotionSystem;

impl<'s> System<'s> for RelativeMotionSystem {
    type SystemData = (
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (velocities, mut transforms, time): Self::SystemData) {
        for (velocity, transform) in (&velocities, &mut transforms).join() {
            transform.prepend_translation_x(velocity.0 * time.delta_seconds());
            transform.prepend_translation_y(velocity.1 * time.delta_seconds());
        }
    }
}
