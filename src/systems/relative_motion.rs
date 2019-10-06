//! The `RelativeMotionSystem` is used to move every entity with a velocity and
//! a transform according to the current time scale of the world. This is an
//! abstraction to combine motion updates with the current time scale to avoid
//! the possibility of updating in its absence.

use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, System, WriteStorage},
};

use crate::components::RelativeLocomotor;
use crate::resources::TimeScale;

pub struct RelativeMotionSystem;

impl<'s> System<'s> for RelativeMotionSystem {
    // TODO: is this idiomatic? Can I package these in some more convenient
    // structure?
    type SystemData = (
        WriteStorage<'s, RelativeLocomotor>,
        WriteStorage<'s, Transform>,
        Read<'s, TimeScale>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        // TODO: is this idiomatic? Can I package these in some more convenient
        // structure?
        (
            mut locomotors,
            mut transforms,
            time_scale,
            time
        ): Self::SystemData
    ) {
        let entity_iter = (&mut locomotors, &mut transforms).join();
        for (locomotor, transform) in entity_iter {
            // We simply multiply the current time scale by the amount of time
            // that passed.
            let scaled_time = time_scale.0 * time.delta_seconds();

            // We calculate the entity's new position, scaling by the time that
            // passed.
            let new_pos = locomotor.pos + (locomotor.velocity * scaled_time);

            // We update the entity's transform.
            transform.set_translation(new_pos.into());

            // Now we update the `old_pos` and `pos` fields on the relative
            // locomotor.
            locomotor.old_pos = locomotor.pos;
            locomotor.pos = new_pos;
        }
    }
}
