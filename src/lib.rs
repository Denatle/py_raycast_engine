mod camera_state;
mod utils;
mod game_state;
mod settings_state;

use game_state::GameState;
use camera_state::CameraState;

use pyo3::prelude::*;
use crate::settings_state::SettingsState;


/// create_game_state(map: list[int], player_x: float, player_y: float, player_angle: float) -> builtins.GameState
/// --
///
/// Returns GameState
#[pyfunction]
fn create_game_state(map: [u32; 32], player_x: f32, player_y: f32, player_angle: f32) -> PyResult<GameState> {
    Ok(GameState::new(map, player_x, player_y, player_angle))
}

/// edit_game_state(game_state: builtins.GameState, player_x: float, player_y: float, player_angle: float) -> builtins.GameState
/// --
///
/// Returns edited GameState
#[pyfunction]
fn edit_game_state(game_state: &PyAny, player_x: f32, player_y: f32, player_angle: f32) -> PyResult<GameState> {
    let game_state: PyRef<GameState> = game_state.extract().unwrap();
    let game_state = GameState::new(game_state.get_map(), player_x, player_y, player_angle);
    Ok(game_state)
}

/// create_settings_state(fov: float | None, size: int | None, wall_height: float | None) -> builtins.SettingsState
/// --
///
/// Returns SettingsState
#[pyfunction]
fn create_settings_state(fov: Option<f32>, size: Option<usize>, wall_height: Option<f32>) -> PyResult<SettingsState> {
    let mut settings = SettingsState::default();
    if let Some(fov) = fov { settings.fov = fov }
    if let Some(size) = size { settings.size = size }
    if let Some(wall_height) = wall_height { settings.wall_height = wall_height }
    Ok(settings)
}

/// get_view(game_state: builtins.GameState, settings_state: builtins.SettingsState) -> list[int, int, int]
/// --
///
/// Returns view of player camera
#[pyfunction]
fn get_view(game_state: &PyAny, settings_state: &PyAny) -> PyResult<Vec<(usize, i32, u32)>> {
    let game_state: PyRef<GameState> = game_state.extract().unwrap();
    let settings_state: PyRef<SettingsState> = settings_state.extract().unwrap();
    let mut result: Vec<(usize, i32, u32)> = vec![];
    for (x, wall_height) in game_state.camera_state.get_view(game_state.get_map(), &settings_state).iter().enumerate() {
        result.push((x, settings_state.size as i32 / 2 - (wall_height / 2), *wall_height as u32));
    }
    Ok(result)
}

#[pymodule]
fn raycast(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_view, m)?)?;
    m.add_function(wrap_pyfunction!(create_game_state, m)?)?;
    m.add_function(wrap_pyfunction!(edit_game_state, m)?)?;
    m.add_function(wrap_pyfunction!(create_settings_state, m)?)?;
    m.add_class::<GameState>()?;
    m.add_class::<CameraState>()?;
    m.add_class::<SettingsState>()?;
    Ok(())
}
