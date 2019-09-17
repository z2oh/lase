#[allow(dead_code)]
pub fn normalize(v: [f32; 2]) -> [f32; 2] {
    let x = v[0];
    let y = v[1];
    let magnitude = (x*x + y*y).sqrt();
    if magnitude == 0.0 {
        [x, y]
    } else {
        [x / magnitude, y / magnitude]
    }
}

#[allow(dead_code)]
pub fn scale(v: [f32; 2], s: f32) -> [f32; 2] {
    [v[0] * s, v[1] * s]
}

#[allow(dead_code)]
pub fn clamp(v: f32, min: f32, max: f32) -> f32 {
    if v > max {
        max
    } else if v < min {
        min
    } else {
        v
    }
}

#[allow(dead_code)]
pub fn sign(v: f32) -> f32 {
    if v < 0.0 {
        -1.0
    } else if v > 0.0 {
        1.0
    } else {
        0.0
    }
}
