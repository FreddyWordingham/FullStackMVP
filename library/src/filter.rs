use ndarray::{Array2, Zip};

use crate::Gradient;

pub fn apply_shadowmap(data: &mut Array2<[u8; 3]>, shadowmap: &Array2<f64>) {
    Zip::from(data).and(shadowmap).par_for_each(|rgb, &shadow| {
        for c in 0..3 {
            rgb[c] = (rgb[c] as f64 * (1.0 - shadow)) as u8;
        }
    });
}

pub fn greyscale_data(data: Array2<f64>) -> Array2<u8> {
    let mut greyscale_data = Array2::zeros(data.dim());
    Zip::from(&mut greyscale_data)
        .and(&data)
        .par_for_each(|grey, &val| {
            *grey = (255.0 * val) as u8;
        });
    greyscale_data
}

pub fn colour_data(data: Array2<f64>, cmap: &Gradient) -> Array2<[u8; 3]> {
    let mut colour_data = Array2::from_elem(data.dim(), [0u8; 3]);
    Zip::from(&mut colour_data)
        .and(&data)
        .par_for_each(|rgb, &val| {
            *rgb = cmap.sample(val as f32).into_format().into();
        });
    colour_data
}
