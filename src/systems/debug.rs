//! This is a very unstable debugging system. Currently, when enabled, this
//! system will simply print out the time between update frames
//! (`time.delta_time()`).

// TODO: lots of cool things could go in here, including but not limited to:
//  [x] print delta time
//  [ ] configurable on-off states for any of the features listed below
//  [ ] display on screen diagnostics (incl. fps/delta time) (F3 information)
//  [ ] display velocity vectors
//  [ ] display bounding boxes
//  [ ] display world coords
//  etc.

// TODO: some of the features above require read access to entities, but only
// entities with certain components when certain features are enabled. how to do
// this? `Option<ReadStorage<_>>`?

use amethyst::core::Time;
use amethyst::ecs::{Read, System};

#[derive(Default)]
pub struct DebugSystem;

impl<'s> System<'s> for DebugSystem {
    type SystemData = Read<'s, Time>;

    fn run(
        &mut self,
        time: Self::SystemData,
    ) {
        println!("delta time: {}", time.delta_seconds())
    }
}
