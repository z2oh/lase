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

// TODO: factor into collision detection module.
// TODO: standardize on rectangle type.

/// Determines if the line segment from `l1` to `l2` intersects the rectangle
/// specified by `r`. This rectangle is stored as a 4-tuple of the rectangle's
/// top left x-coordinate, top left y-coordinate, width, and height.
pub fn line_intersects_rect(
    l1: (f32, f32),
    l2: (f32, f32),
    r: (f32, f32, f32, f32)
) -> bool {
    // Compute the four corner points of the rectangle. The order in which these
    // points are computed lends to a clockwise traversal of the rectangle's
    // edges.
    let r1 = (r.0, r.1);
    let r2 = (r.0 + r.2, r.1);
    let r3 = (r.0 + r.2, r.1 + r.3);
    let r4 = (r.0, r.1 + r.3);

    line_intersects_line(l1, l2, r1, r2) ||
    line_intersects_line(l1, l2, r2, r3) ||
    line_intersects_line(l1, l2, r3, r4) ||
    line_intersects_line(l1, l2, r4, r1)
}

/// Determines if the line segment from `a` to `b` intersects the line
/// segment from  `c` to `d`.
// TODO: document this algorithm.
pub fn line_intersects_line(a: (f32, f32), b: (f32, f32), c: (f32, f32), d: (f32,  f32)) -> bool {
    let o1 = triplet_orientation((a, b, c));
    let o2 = triplet_orientation((a, b, d));
    let o3 = triplet_orientation((c, d, a));
    let o4 = triplet_orientation((c, d, b));

    o1 != o2 && o3 != o4 ||
    o3 == TripletOrientation::Colinear && on_segment((c, a, d)) ||
    o4 == TripletOrientation::Colinear && on_segment((c, b, d))
}

// TODO: document this algorithm.
pub fn on_segment((p, q, r): ((f32, f32), (f32, f32), (f32, f32))) -> bool {
    q.0 <= p.0.max(r.0) &&
    q.0 >= p.0.min(r.0) &&
    q.1 <= p.1.max(r.1) &&
    q.1 >= p.1.min(r.1)
}

/// The three cases of possible orientations for triplets of points.
#[derive(PartialEq)]
pub enum TripletOrientation {
    Colinear,
    Clockwise,
    Counterclockwise,
}

/// Calculates the orientation of the path from `p` to `q` to `r`.
///
/// # Examples
///
/// ```
/// // The path here from the origin, up the x-axis, and then to the y-axis
/// // forms a counterclockwise orientation.
/// assert_eq!(triplet_orientation((0.0, 0.0), (1.0, 0.0), (0.0, 1.0)),
/// // By going up the y-axis first and then along the x-axis, we now have a
/// // clockwise orientation.
/// assert_eq!(triplet_orientation((0.0, 0.0), (0.0, 1.0), (1.0, 0.0)),
/// TripletOrientation::Clockwise);
/// // Colinear points are a special case.
/// assert_eq!(triplet_orientation((-1.0, -1.0), (0.0, 0.0), (1.0, 1.0)),
/// TripletOrientation::Colinear);
/// ```
pub fn triplet_orientation((p, q, r): ((f32, f32), (f32, f32), (f32, f32))) -> TripletOrientation {
    let v = (q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 -  q.1);
    if v == 0.0 {
        TripletOrientation::Colinear
    } else if v > 0.0 {
        TripletOrientation::Clockwise
    } else {
        TripletOrientation::Counterclockwise
    }
}
