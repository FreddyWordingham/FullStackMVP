use ndarray::{Array2, Zip};

use crate::{complex::Complex, Settings};

pub fn sample_point(c: Complex, max_iter: i32) -> i32 {
    let mut z = Complex::zero();
    for n in 0..max_iter {
        z = z * z + c.clone();
        if z.norm_squared() > 4.0 {
            return n;
        }
    }
    return max_iter;
}

pub fn multi_sample(c: Complex, max_iter: i32, super_samples: i32, epsilon: f64) -> f64 {
    let start = c + Complex::new(
        epsilon * (1 - super_samples) as f64,
        epsilon * (1 - super_samples) as f64,
    );
    let mut total = 0;
    for i in 0..super_samples {
        let re = epsilon * (i as f64);
        for j in 0..super_samples {
            let im = epsilon * (j as f64);
            total += sample_point(start + Complex::new(re, im), max_iter);
        }
    }
    total as f64 / (super_samples * super_samples) as f64
}

pub fn sample_area(settings: &Settings) -> Array2<f64> {
    let mut data = Array2::zeros(settings.resolution);
    let (height, width) = data.dim();

    let aspect_ratio = width as f64 / height as f64;
    let start =
        settings.centre + Complex::new(settings.scale * -0.5, settings.scale / aspect_ratio * -0.5);
    let delta = settings.scale / (width - 1).max(1) as f64;
    let epsilon = delta / (2 * settings.super_samples) as f64;

    Zip::indexed(&mut data).par_for_each(|(yi, xi), pixel| {
        let y = start.im + (delta * yi as f64);
        let x = start.re + (delta * xi as f64);
        let c = Complex::new(x, y);
        *pixel = multi_sample(
            c,
            settings.max_iter as i32,
            settings.super_samples as i32,
            epsilon,
        );
    });

    data
}
