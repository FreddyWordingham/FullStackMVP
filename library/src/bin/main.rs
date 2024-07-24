use enterpolation::{linear::Linear, Equidistant, Generator, Identity};
use my_library::{render_colour_image, render_greyscale_image, sample_area, Complex};
use ndarray::{Array2, Zip};
use palette::LinSrgb;

type Gradient = Linear<Equidistant<f32>, Vec<LinSrgb>, Identity>;

fn main() {
    let centre = Complex::new(-1.393, 0.00999);
    let scale = 1.0e-2;
    let super_samples = 2;
    let max_iter = 100;
    let resolution = [1000, 2000];

    let colours = ["#FF0000", "#007FFF"];
    let cmap: Gradient = Linear::builder()
        .elements(
            colours
                .iter()
                .map(|c| {
                    let r = u8::from_str_radix(&c[1..3], 16).unwrap() as f32 / 255.0;
                    let g = u8::from_str_radix(&c[3..5], 16).unwrap() as f32 / 255.0;
                    let b = u8::from_str_radix(&c[5..7], 16).unwrap() as f32 / 255.0;
                    LinSrgb::new(r, g, b)
                })
                .collect::<Vec<_>>(),
        )
        .equidistant::<f32>()
        .normalized()
        .build()
        .unwrap();

    let data = sample_area(centre, scale, resolution, super_samples, max_iter);

    let greyscale = greyscale_data(data.clone(), max_iter);
    let mut colour = colour_data(data.clone(), max_iter, &cmap);

    let light_direction = [0.0, 1.0, 1.0]; // Example light direction
    let shadowmap = calculate_shadowmap(&data, light_direction);

    Zip::from(&mut colour)
        .and(&shadowmap)
        .par_for_each(|rgb, &shadow| {
            for c in 0..3 {
                rgb[c] = (rgb[c] as f64 * (1.0 - shadow)) as u8;
            }
        });

    let greyscale_image = render_greyscale_image(&greyscale);
    let colour_image = render_colour_image(&colour);

    greyscale_image.save("output/greyscale.png").unwrap();
    colour_image.save("output/colour.png").unwrap();
}

fn greyscale_data(data: Array2<f64>, max_iter: i32) -> Array2<u8> {
    let mut greyscale_data = Array2::zeros(data.dim());
    Zip::from(&mut greyscale_data)
        .and(&data)
        .par_for_each(|grey, &val| {
            *grey = (255.0 * (val / max_iter as f64)) as u8;
        });
    greyscale_data
}

fn colour_data(data: Array2<f64>, max_iter: i32, cmap: &Gradient) -> Array2<[u8; 3]> {
    let mut colour_data = Array2::from_elem(data.dim(), [0u8; 3]);
    Zip::from(&mut colour_data)
        .and(&data)
        .par_for_each(|rgb, &val| {
            let x = val / max_iter as f64;
            *rgb = cmap.gen(x as f32).into_format().into();
        });
    colour_data
}

fn calculate_shadowmap(heightmap: &Array2<f64>, light_direction: [f64; 3]) -> Array2<f64> {
    let mut shadowmap = Array2::zeros(heightmap.dim());

    let (height, width) = heightmap.dim();
    let light_vec =
        nalgebra::Vector3::new(light_direction[0], light_direction[1], light_direction[2])
            .normalize();

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let dzdx = (heightmap[[y, x + 1]] - heightmap[[y, x - 1]]) / 2.0;
            let dzdy = (heightmap[[y + 1, x]] - heightmap[[y - 1, x]]) / 2.0;
            let normal = nalgebra::Vector3::new(-dzdx, -dzdy, 1.0).normalize();
            let intensity = normal.dot(&light_vec).max(0.0);
            shadowmap[[y, x]] = intensity;
        }
    }

    shadowmap
}
