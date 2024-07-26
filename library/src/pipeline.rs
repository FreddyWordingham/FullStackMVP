use ndarray::Array2;

use crate::{filter, lighting, mandelbrot, math, settings::Settings};

pub fn run(settings: &Settings) -> (Array2<f64>, Array2<f64>, Array2<u8>, Array2<[u8; 3]>) {
    // Sample complex plane
    let mut samples = mandelbrot::sample_area(&settings);
    math::normalise(&mut samples);

    // Calculate lighting
    let mut light_map = lighting::calculate_light_map(
        &(samples.clone() * settings.height_scaling),
        settings.light_direction,
    );
    math::normalise(&mut light_map);

    // Apply filters
    let greyscale = filter::greyscale_data(samples.clone(), &settings.greyscale_expression);
    let colour = filter::colour_data(
        samples.clone(),
        &settings.colour_expression,
        &settings.gradient,
    );

    (samples, light_map, greyscale, colour)
}
