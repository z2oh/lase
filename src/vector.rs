//! This module largely wraps a bunch of common `nalgebra` types with more
//! convenient names that can be glob-imported via `vector::prelude::*`. There
//! are some vector-related helper functions as well.

/// The storage type. This is an attempt to make it easier to switch backing
/// storage types in the future, if it becomes the case that `f64` is needed
/// instead of `f32`. I imagine `f32` will be enough for anything, but you never
/// know.
pub type StorageTy = f32;

/// A 2D point in geometric space. The underlying `Vec2` may be accessed with
/// the `coords` property.
pub type Point2 = nalgebra::Point2<StorageTy>;
/// A 2D vector, with all the benefits it brings.
pub type Vec2 = nalgebra::Vector2<StorageTy>;
/// A 3D vector, with all the benefits it brings.
pub type Vec3 = nalgebra::Vector3<StorageTy>;
/// A 2D rotation matrix.
pub type Rot2 = nalgebra::Rotation2<StorageTy>;
#[allow(dead_code)]
/// A 3D rotation matrix.
pub type Rot3 = nalgebra::Rotation3<StorageTy>;
/// A unit quaternion, used by transforms to represent rotation.
pub type Quaternion = nalgebra::UnitQuaternion<StorageTy>;

/// PI. This constant must always be based on `StorageTy`.
pub const PI: f32 = std::f32::consts::PI;

pub mod prelude {
    pub use super::StorageTy;

    pub use super::Point2;
    pub use super::Vec2;
    pub use super::Vec3;
    pub use super::Rot2;
    pub use super::Rot3;
    pub use super::Quaternion;

    pub use super::add_dim;

    // Reexport particularly useful constants.
    pub use super::PI;
}

/// Converts a Vec2 into a Vec3. This is primarily used when going from 2D
/// coordinates to 3D coordinates. The default assumption is that everything
/// will be in the xy plane, so a default z value of `0.0` is chosen.
pub fn add_dim(vec2: Vec2) -> Vec3 {
    use nalgebra::dimension::{U1, U3};

    vec2.fixed_resize::<U3, U1>(0.0)
}
