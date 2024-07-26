use meval::Expr;

use crate::{complex::Complex, gradient::Gradient};

pub struct Settings {
    pub centre: Complex,
    pub scale: f64,
    pub super_samples: u32,
    pub max_iter: u32,
    pub resolution: [usize; 2],
    pub light_direction: [f64; 3],
    pub height_scaling: f64,
    pub gradient: Gradient,
    pub colour_expression: Expr,
    pub greyscale_expression: Expr,
}

impl Settings {
    pub fn new(
        centre: Complex,
        scale: f64,
        super_samples: u32,
        max_iter: u32,
        resolution: [usize; 2],
        light_direction: [f64; 3],
        height_scaling: f64,
        gradient: Gradient,
        colour_expression: Expr,
        greyscale_expression: Expr,
    ) -> Self {
        Self {
            centre,
            scale,
            super_samples,
            max_iter,
            resolution,
            light_direction,
            height_scaling,
            gradient,
            colour_expression,
            greyscale_expression,
        }
    }
}
