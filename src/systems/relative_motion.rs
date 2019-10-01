//! The `RelativeMotionSystem` is used to move every entity with a velocity and
//! a transform according to the current time scale of the world. This is an
//! abstraction to combine motion updates with the current time scale to avoid
//! the possibility of updating in its absence.

use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::Velocity;
use crate::resources::TimeScale;

pub struct RelativeMotionSystem;

impl<'s> System<'s> for RelativeMotionSystem {
    // TODO: is this idiomatic? Can I package these in some more convenient
    // structure?
    type SystemData = (
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, TimeScale>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        // TODO: is this idiomatic? Can I package these in some more convenient
        // structure?
        (
            velocities,
            mut transforms,
            time_scale,
            time
        ): Self::SystemData
    ) {
        for (velocity, transform) in (&velocities, &mut transforms).join() {
            // We simply multiply the current time scale by the amount of time
            // that passed.
            let scaled_time = time_scale.0 * time.delta_seconds();
            transform.prepend_translation_x(velocity.0 * scaled_time);
            transform.prepend_translation_y(velocity.1 * scaled_time);
        }
    }
}
