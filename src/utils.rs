use libm::{sqrtf};

pub fn distance(a: f32, b: f32) -> f32 {
    sqrtf((a * a) + (b * b))
}

/// Check if the map contains a wall at a point.
pub fn point_in_wall(x: f32, y: f32, map: [u32; 32]) -> bool {
    match map.get(y as usize) {
        Some(line) => (line & (0b1 << x as usize)) != 0,
        None => true,
    }
}
