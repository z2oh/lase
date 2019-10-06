//! This module holds a small number of utility functions, mostly for
//! manipulating numbers. This should be considered an "unstable" module;
//! functions here are very likely to be factored out once the scope of their
//! potential use is better understood.

pub mod prelude {
    use rand;
    pub use rand::random;

    use nalgebra;
    pub use nalgebra::clamp;
}
