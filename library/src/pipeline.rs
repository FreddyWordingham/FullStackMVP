use ndarray::Array2;

use crate::{filter, lighting, mandelbrot, math, settings::Settings};

pub fn run(settings: &Settings) -> (Array2<f64>, Array2<f64>, Array2<u8>, Array2<[u8; 3]>) {
    // Sample complex plane
    let mut samples = mandelbrot::sample_area(&settings);
    math::normalise(&mut samples);

    // Calculate shadow_map
    let mut shadow_map = lighting::calculate_shadow_map(
        &(samples.clone() * settings.height_scaling),
        settings.light_direction,
    );
    math::normalise(&mut shadow_map);

    // Apply filters
    let greyscale = filter::greyscale_data(samples.clone(), &settings.greyscale_expression);
    let colour = filter::colour_data(
        samples.clone(),
        &settings.colour_expression,
        &settings.gradient,
    );

    (samples, shadow_map, greyscale, colour)
}
