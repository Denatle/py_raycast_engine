use std::f32::consts::PI;
use pyo3::prelude::*;

#[pyclass]
#[derive(FromPyObject)]
pub struct SettingsState {
    // The player's field of view.
    pub fov: f32,
    // The size of view
    pub size: usize,
    // The angle between each ray.
    pub angle_step: f32,
    // A magic number.
    pub wall_height: f32,
}

impl Default for SettingsState {
    fn default() -> Self {
        let size = 500;
        let fov = PI / 2.7;
        SettingsState {
            fov,
            size,
            angle_step: fov / size as f32,
            wall_height: size as f32 / 1.6,
        }
    }
}