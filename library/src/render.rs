use image::{GrayImage, RgbImage};
use ndarray::Array2;

/// Render the given array of values as an image.
pub fn render_greyscale_image(data: &Array2<u8>) -> GrayImage {
    let (width, height) = data.dim();

    GrayImage::from_vec(
        height as u32,
        width as u32,
        data.as_slice().unwrap().to_vec(),
    )
    .expect("Container should have the right size for the image dimensions.")
}

/// Render the given array of RGB values as an image.
pub fn render_colour_image(data: &Array2<[u8; 3]>) -> RgbImage {
    let (width, height) = data.dim();

    RgbImage::from_vec(
        height as u32,
        width as u32,
        data.as_slice()
            .unwrap()
            .to_vec()
            .into_iter()
            .flat_map(|x| x.to_vec())
            .collect(),
    )
    .expect("Container should have the right size for the image dimensions.")
}
