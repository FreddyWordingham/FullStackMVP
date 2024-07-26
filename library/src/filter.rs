use meval::Expr;
use ndarray::{Array2, Zip};

use crate::gradient::Gradient;

pub fn apply_shadow_map(data: &mut Array2<[u8; 3]>, shadow_map: &Array2<f64>) {
    Zip::from(data)
        .and(shadow_map)
        .par_for_each(|rgb, &shadow| {
            for c in 0..3 {
                rgb[c] = (rgb[c] as f64 * (1.0 - shadow)) as u8;
            }
        });
}

pub fn greyscale_data(data: Array2<f64>, f: &Expr) -> Array2<u8> {
    let mut greyscale_data = Array2::zeros(data.dim());
    Zip::from(&mut greyscale_data)
        .and(&data)
        .par_for_each(|grey, &val| {
            let transform = f.clone().bind("x").unwrap();
            *grey = (transform(val).clamp(0.0, 1.0) * 255.0) as u8;
        });
    greyscale_data
}

pub fn colour_data(data: Array2<f64>, f: &Expr, cmap: &Gradient) -> Array2<[u8; 3]> {
    let mut colour_data = Array2::from_elem(data.dim(), [0u8; 3]);
    Zip::from(&mut colour_data)
        .and(&data)
        .par_for_each(|rgb, &val| {
            let transform = f.clone().bind("x").unwrap();
            *rgb = cmap.sample(transform(val) as f32).into_format().into();
        });
    colour_data
}
