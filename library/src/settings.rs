use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

use crate::{Complex, Gradient};

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

pub struct Settings {
    pub centre: Complex,
    pub scale: f64,
    pub super_samples: u32,
    pub max_iter: u32,
    pub resolution: [usize; 2],
    pub light_direction: [f64; 3],
    pub gradient: Gradient,
}

impl Settings {
    pub fn load(filepath: &str) -> Result<Self> {
        // Open the file
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);

        // Deserialize the YAML into a Config struct
        let config: Config = serde_yaml::from_reader(reader)?;

        Ok(Self {
            centre: Complex::new(config.centre[0], config.centre[1]),
            scale: config.scale,

            super_samples: config.super_samples,
            max_iter: config.max_iter,
            resolution: config.resolution,
            light_direction: config.light_direction,

            gradient: Gradient::new(
                &config
                    .colours
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>(),
            ),
        })
    }
}
