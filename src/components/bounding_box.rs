//! This component holds the width and height of a bounding box. This component
//! is expected to be added to an entity with a transform, to give the box an
//! implicit position.
use amethyst::ecs::prelude::{Component, DenseVecStorage};

// TODO: group this in some sort of "collidable" component?
// TODO: is this idiomatic?
pub struct BoundingBox(pub f32, pub f32);

impl Component for BoundingBox {
    // TODO: investigate storage types.
    type Storage = DenseVecStorage<Self>;
}
