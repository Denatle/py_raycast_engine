use crate::camera_state::CameraState;
use pyo3::prelude::*;

#[pyclass]
#[derive(FromPyObject)]
pub struct GameState {
    pub camera_state: CameraState,
    map: [u32; 32],
}

#[pymethods]
impl GameState {
    #[getter]
    pub fn get_map(&self) -> [u32; 32] {
        self.map
    }
    #[new]
    pub fn new(map: [u32; 32], player_x: f32, player_y: f32, player_angle: f32) -> Self {
        GameState {
            camera_state: CameraState {
                player_x,
                player_y,
                player_angle,
            },
            map,
        }
    }
}