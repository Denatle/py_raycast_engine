mod camera_state;
mod utils;
mod game_state;

use game_state::GameState;
use camera_state::CameraState;

use pyo3::prelude::*;
use core::f32::consts::{PI};

// The player's field of view.
const FOV: f32 = PI / 2.7;
// Half the player's field of view.
const HALF_FOV: f32 = FOV * 0.5;
// The size of view
pub const SIZE: usize = 500;
// Half the size of view
const HALF_SIZE: i32 = SIZE as i32 / 2;
// The angle between each ray.
pub const ANGLE_STEP: f32 = FOV / SIZE as f32;
// A magic number.
pub const WALL_HEIGHT: f32 = SIZE as f32 / 1.6;


const MAP: [u32; 32] = [
    0b11111111111111111111111111111111,
    0b10000000000000000000000000000001,
    0b10100010010000000000000010000001,
    0b10000000000000000000000000000001,
    0b10000000001001000000000000000001,
    0b10001000000000000000000001000001,
    0b10000000000100000000000000000001,
    0b10010000000000000000000000000001,
    0b10000000000000000000000100000001,
    0b10010000010000000000000000000001,
    0b10000000000100000000000000000001,
    0b10000000000000000000000000000001,
    0b10000100000000000000000100000001,
    0b10000000100000000000000000000001,
    0b10000000000000000000000100000001,
    0b10000001000000000000000000000001,
    0b10000000000000000000000000000001,
    0b10000000000000000000000100000001,
    0b10000000001000010000000000000001,
    0b10000000000000000000100000000001,
    0b10000000000000000000000000000001,
    0b10000000000000000000000000100001,
    0b10000000000000010000000000000001,
    0b10000000000000000000000000000001,
    0b10000000100000000000000010000001,
    0b10000010000000000000100000000001,
    0b10000000000000000000000000000001,
    0b10000000010000000000000000000001,
    0b10000000000000001000000000000001,
    0b10000100000000000000000000000001,
    0b10000000000000000000000000000001,
    0b11111111111111111111111111111111,
];


#[pyfunction]
fn create_state(map: [u32; 32], player_x: f32, player_y: f32, player_angle: f32) -> PyResult<GameState> {
    Ok(GameState::new(map, player_x, player_y, player_angle))
}

#[pyfunction]
fn return_view(game_state: &PyAny) -> PyResult<Vec<(usize, i32, u32)>> {
    let game_state: PyRef<GameState> = game_state.extract().unwrap();
    let mut result: Vec<(usize, i32, u32)> = vec![];
    for (x, wall_height) in game_state.camera_state.get_view(game_state.get_map()).iter().enumerate() {
        result.push((x, HALF_SIZE - (wall_height / 2), *wall_height as u32));
    }
    Ok(result)
}

#[pymodule]
fn raycast(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(return_view, m)?)?;
    m.add_function(wrap_pyfunction!(create_state, m)?)?;
    m.add_class::<GameState>()?;
    m.add_class::<CameraState>()?;
    Ok(())
}
