//! This module holds anything related to collision detection and the supporting
//! geometric utilities.
//!
//! The submodules of this module export groups of related items that can be
//! conveniently glob imported. These submodules can be viewed as the public
//! APIs of various systems of functionality within the larger module.
use crate::vector::prelude::*;

/// This module reexports the Rectangle type, as well as the public facing box
/// collision detection functions.
pub mod box_collision {
    pub use super::Rect;
    pub use super::BoundingBox;

    pub use super::line_intersects_line;
    pub use super::line_intersects_rect;
}

/// A bounding box is simply represented as a vector from the center of the
/// object to its top right corner. It is expected that an object with a
/// `BoundingBox` will have an `amethyst::core::Transform` component as well.
#[derive(Clone, Copy, Debug)]
pub struct BoundingBox(pub Vec2);

impl From<Vec2> for BoundingBox {
    fn from(vec2: Vec2) -> Self {
        Self(vec2)
    }
}

/// A basic rectangle type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pos: Point2,
    width: StorageTy,
    height: StorageTy,
}

impl Rect {
    pub fn from_bounding_box(center: Point2, bb: BoundingBox) -> Self {
        let bb_diag = bb.0 * 2.0;
        Self {
            pos: center - bb.0,
            width: bb_diag[0],
            height: bb_diag[1],
        }
    }
}

/// The three cases of possible orientations for triplets of points.
#[derive(PartialEq)]
pub enum TripletOrientation {
    Colinear,
    Clockwise,
    Counterclockwise,
}

/// Determines if the line segment from `l1` to `l2` intersects the rectangle
/// specified by `r`. This rectangle is stored as a 4-tuple of the rectangle's
/// top left x-coordinate, top left y-coordinate, width, and height.
pub fn line_intersects_rect(
    p1: Point2,
    p2: Point2,
    r: Rect,
) -> bool {
    // Compute the four corner points of the rectangle. The order in which these
    // points are computed lends to a counterclockwise traversal of the
    // rectangle's edges.
    let width_vec = r.width * Vec2::x();
    let height_vec = r.height * Vec2::y();
    let r1 = r.pos;
    let r2 = r.pos + width_vec;
    let r3 = r.pos + width_vec + height_vec;
    let r4 = r.pos + height_vec;

    line_intersects_line(p1, p2, r1, r2) ||
    line_intersects_line(p1, p2, r2, r3) ||
    line_intersects_line(p1, p2, r3, r4) ||
    line_intersects_line(p1, p2, r4, r1)
}

/// Determines if the line segment from `a` to `b` intersects the line
/// segment from  `c` to `d`.
// TODO: document this algorithm.
pub fn line_intersects_line(a: Point2, b: Point2, c: Point2, d: Point2) -> bool {
    let o1 = triplet_orientation(a, b, c);
    let o2 = triplet_orientation(a, b, d);
    let o3 = triplet_orientation(c, d, a);
    let o4 = triplet_orientation(c, d, b);

    o1 != o2 && o3 != o4 ||
    o3 == TripletOrientation::Colinear && on_segment(c, a, d) ||
    o4 == TripletOrientation::Colinear && on_segment(c, b, d)
}

// TODO: document this algorithm.
pub fn on_segment(p: Point2, q: Point2, r: Point2) -> bool {
    q[0] <= p[0].max(r[0]) &&
    q[0] >= p[0].min(r[0]) &&
    q[1] <= p[1].max(r[1]) &&
    q[1] >= p[1].min(r[1])
}

/// Calculates the orientation of the path from `p` to `q` to `r`.
///
/// # Examples
///
/// ```
/// // The path here from the origin, up the x-axis, and then to the y-axis
/// // forms a counterclockwise orientation.
/// assert_eq!(
///     triplet_orientation(
///         Point2::new(0.0, 0.0),
///         Point2::new(1.0, 0.0),
///         Point2::new(0.0, 1.0),
///     ),
///     TripletOrientation::CounterClockwise
/// );
/// // By going up the y-axis first and then along the x-axis, we now have a
/// // clockwise orientation.
/// assert_eq!(
///     triplet_orientation(
///         Point2::new(0.0, 0.0),
///         Point2::new(0.0, 1.0),
///         Point2::new(1.0, 0.0),
///     ),
///     TripletOrientation::Clockwise
/// );
/// // Colinear points are a special case.
/// assert_eq!(
///     triplet_orientation(
///         Point2::new(-1.0, -1.0),
///         Point2::new(0.0, 0.0),
///         Point2::new(1.0, 1.0),
///     ),
///     TripletOrientation::Colinear
/// );
/// ```
pub fn triplet_orientation(p: Point2, q: Point2, r: Point2) -> TripletOrientation {
    let v = (q[1] - p[1]) * (r[0] - q[0]) - (q[0] - p[0]) * (r[1] -  q[1]);
    if v == 0.0 {
        TripletOrientation::Colinear
    } else if v > 0.0 {
        TripletOrientation::Clockwise
    } else {
        TripletOrientation::Counterclockwise
    }
}
