use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    centre: [f64; 2],
    scale: f64,
    super_samples: u32,
    max_iter: u32,
    resolution: [usize; 2],
    light_direction: [f64; 3],
    colours: Vec<String>,
}
