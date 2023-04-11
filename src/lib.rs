use pyo3::prelude::*;
use libm::{ceilf, cosf, fabsf, floorf, sqrtf, tanf};
use core::f32::consts::{PI, FRAC_PI_2};

const FOV: f32 = PI / 2.7;
// The player's field of view.
const HALF_FOV: f32 = FOV * 0.5;
// Half the player's field of view.
const ANGLE_STEP: f32 = FOV / 500.0;
// The angle between each ray.
const WALL_HEIGHT: f32 = 312.5; // A magic number.

struct State {
    player_x: f32,
    player_y: f32,
    player_angle: f32,
}

impl State {
    /// Returns the nearest wall the ray intersects with on a vertical grid line.
    fn vertical_intersection(&self, angle: f32) -> f32 {
        // This tells you if the angle is "facing up"
        // regardless of how big the angle is.
        let right = fabsf(floorf((angle - FRAC_PI_2) / PI) % 2.0) != 0.0;

        // first_y and first_x are the first grid intersections
        // that the ray intersects with.
        let first_x = if right {
            ceilf(self.player_x) - self.player_x
        } else {
            floorf(self.player_x) - self.player_x
        };
        let first_y = -tanf(angle) * first_x;

        // dy and dx are the "ray extension" values mentioned earlier.
        let dx = if right { 1.0 } else { -1.0 };
        let dy = dx * -tanf(angle);

        // next_x and next_y are mutable values which will keep track
        // of how far away the ray is from the player.
        let mut next_x = first_x;
        let mut next_y = first_y;

        // This is the loop where the ray is extended until it hits
        // the wall. It's not an infinite loop as implied in the
        // explanation, instead it only goes from 0 to 256.
        //
        // This was chosen because if something goes wrong and the
        // ray never hits a wall (which should never happen) the
        // loop will eventually quit and the game will keep on running.
        for _ in 0..256 {
            // current_x and current_y are where the ray is currently
            // on the map, while next_x and next_y are relative
            // coordinates, current_x and current_y are absolute
            // points.
            let current_x = if right {
                next_x + self.player_x
            } else {
                next_x + self.player_x - 1.0
            };
            let current_y = next_y + self.player_y;

            // Tell the loop to quit if we've just hit a wall.
            if point_in_wall(current_x, current_y) {
                break;
            }

            // if we didn't hit a wall on this extension add
            // dx and dy to our current position and keep going.
            next_x += dx;
            next_y += dy;
        }

        // return the distance from next_x and next_y to the player.
        distance(next_x, next_y)
    }
}

impl State {
    /// Returns the nearest wall the ray intersects with on a horizontal grid line.
    fn horizontal_intersection(&self, angle: f32) -> f32 {
        // This tells you if the angle is "facing up"
        // regardless of how big the angle is.
        let up = fabsf(floorf(angle / PI) % 2.0) != 0.0;

        // first_y and first_x are the first grid intersections
        // that the ray intersects with.
        let first_y = if up {
            ceilf(self.player_y) - self.player_y
        } else {
            floorf(self.player_y) - self.player_y
        };
        let first_x = -first_y / tanf(angle);

        // dy and dx are the "ray extension" values mentioned earlier.
        let dy = if up { 1.0 } else { -1.0 };
        let dx = -dy / tanf(angle);

        // next_x and next_y are mutable values which will keep track
        // of how far away the ray is from the player.
        let mut next_x = first_x;
        let mut next_y = first_y;

        // This is the loop where the ray is extended until it hits
        // the wall. It's not an infinite loop as implied in the
        // explanation, instead it only goes from 0 to 256.
        //
        // This was chosen because if something goes wrong and the
        // ray never hits a wall (which should never happen) the
        // loop will eventually break and the game will keep on running.
        for _ in 0..256 {
            // current_x and current_y are where the ray is currently
            // on the map, while next_x and next_y are relative
            // coordinates, current_x and current_y are absolute
            // points.
            let current_x = next_x + self.player_x;
            let current_y = if up {
                next_y + self.player_y
            } else {
                next_y + self.player_y - 1.0
            };

            // Tell the loop to quit if we've just hit a wall.
            if point_in_wall(current_x, current_y) {
                break;
            }

            // if we didn't hit a wall on this extension add
            // dx and dy to our current position and keep going.
            next_x += dx;
            next_y += dy;
        }

        // return the distance from next_x and next_y to the player.
        distance(next_x, next_y)
    }
}

impl State {
    /// Returns 500 wall heights from the player's perspective.
    pub fn get_view(&self) -> [i32; 500] {
        // The player's FOV is split in half by their viewing angle.
        // In order to get the ray's first angle we must
        // add half the FOV to the player's angle to get
        // the edge of the player's FOV.
        let starting_angle = self.player_angle + HALF_FOV;

        let mut walls = [0; 500];

        for (idx, wall) in walls.iter_mut().enumerate() {
            // `idx` is what number ray we are, `wall` is
            // a mutable reference to a value in `walls`.
            let angle = starting_angle - idx as f32 * ANGLE_STEP;

            // Get both the closest horizontal and vertical wall
            // intersections for this angle.
            let h_dist = self.horizontal_intersection(angle);
            let v_dist = self.vertical_intersection(angle);

            // Get the minimum of the two distances and
            // "convert" it into a wall height.

            *wall = (WALL_HEIGHT / (f32::min(h_dist, v_dist) * cosf(angle - self.player_angle))) as i32;
        }

        walls
    }
}

static mut STATE: State = State {
    player_x: 0.0,
    player_y: 0.0,
    player_angle: 0.0,
};

const MAP: [u16; 16] = [
    0b1111111111111111,
    0b1000000000000001,
    0b1010001001000001,
    0b1000000000000001,
    0b1000000000100101,
    0b1000100000000001,
    0b1000000000010001,
    0b1001000000000001,
    0b1000000000000001,
    0b1001000001000001,
    0b1000000000010001,
    0b1000000000000001,
    0b1000010000000001,
    0b1000000010000001,
    0b1000000000000001,
    0b1111111111111111,
];

fn distance(a: f32, b: f32) -> f32 {
    sqrtf((a * a) + (b * b))
}

/// Check if the map contains a wall at a point.
fn point_in_wall(x: f32, y: f32) -> bool {
    match MAP.get(y as usize) {
        Some(line) => (line & (0b1 << x as usize)) != 0,
        None => true,
    }
}

#[pyfunction]
unsafe fn set_state(player_x: f32, player_y: f32, player_angle: f32) {
    STATE.player_x = player_x;
    STATE.player_y = player_y;
    STATE.player_angle = player_angle;
}

#[pyfunction]
unsafe fn return_view() -> PyResult<Vec<(usize, i32, u32)>> {
    let mut result: Vec<(usize, i32, u32)> = vec![];
    for (x, wall_height) in STATE.get_view().iter().enumerate() {
        result.push((x, 250 - (wall_height / 2), *wall_height as u32));
    }
    Ok(result)
}

#[pymodule]
fn raycast(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(return_view, m)?)?;
    m.add_function(wrap_pyfunction!(set_state, m)?)?;
    Ok(())
}
