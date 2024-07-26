use anyhow::Result;
use meval::Expr;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

use crate::{complex::Complex, gradient::Gradient, settings::Settings};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    centre: [f64; 2],
    scale: f64,
    super_samples: u32,
    max_iter: u32,
    resolution: [usize; 2],
    light_direction: [f64; 3],
    height_scaling: f64,
    colours: Vec<String>,
    colour_expression: String,
    greyscale_expression: String,
}

impl Config {
    pub fn load(filepath: &str) -> Result<Config> {
        // Open the file
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);

        // Deserialize the YAML into a Config struct
        let config: Config = serde_yaml::from_reader(reader)?;

        Ok(config)
    }

    pub fn build(&self) -> Settings {
        let colour_expression_func: Expr = self.colour_expression.parse().unwrap();
        let greyscale_expression_func: Expr = self.greyscale_expression.parse().unwrap();

        Settings::new(
            Complex::new(self.centre[0], self.centre[1]),
            self.scale,
            self.super_samples,
            self.max_iter,
            self.resolution,
            self.light_direction,
            self.height_scaling,
            Gradient::new(&self.colours.iter().map(|s| s.as_str()).collect::<Vec<_>>()),
            colour_expression_func,
            greyscale_expression_func,
        )
    }
}
