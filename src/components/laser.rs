//! This component holds properties about a laser.
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Laser {
    // TODO: is this idiomatic? I should probably be using amethyst's color
    // types.
    /// The color of the laser. This is currently applied as a tint over an all
    /// white sprite.
    pub color: (f32, f32, f32)
}

impl Default for Laser {
    fn default() -> Self {
        Self {
            color: (0.0, 0.0, 0.0),
        }
    }
}

impl Component for Laser {
    // TODO: investigate storage types. This component in particular should
    // probably use `VecStorage`.
    type Storage = DenseVecStorage<Self>;
}
