//! This component wraps a `crate::collisions::Rect` object
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub use crate::collisions::BoundingBox;

use crate::vector::StorageTy;

impl BoundingBox {
    /// This function returns the max distance to a corner of the box.
    pub fn dist(self) -> StorageTy {
        self.0.norm()
    }
}

impl Component for BoundingBox {
    // TODO: investigate storage types.
    type Storage = DenseVecStorage<Self>;
}
