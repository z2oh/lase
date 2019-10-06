//! This component holds properties about a laser.
use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::vector::StorageTy;

pub struct Laser {
    // TODO: is this idiomatic? I should probably be using amethyst's color
    // types.
    /// The color of the laser. This is currently applied as a tint over an all
    /// white sprite.
    pub color: (f32, f32, f32),
    pub len: StorageTy,
}

impl Component for Laser {
    // TODO: investigate storage types. This component in particular should
    // probably use `VecStorage`.
    type Storage = DenseVecStorage<Self>;
}
