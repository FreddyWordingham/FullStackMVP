use my_library::{filter, lighting, mandelbrot, math, render, Settings};

fn main() {
    let settings = Settings::load("input/config.yml").expect("Failed to read config file");

    let mut data = mandelbrot::sample_area(&settings);
    let mut shadowmap = lighting::calculate_shadowmap(&data, settings.light_direction);

    math::normalise(&mut shadowmap);

    math::normalise(&mut data);

    let mut colour = filter::colour_data(data.clone(), &settings.gradient);
    let greyscale = filter::greyscale_data(data.clone());

    filter::apply_shadowmap(&mut colour, &shadowmap);

    render::greyscale_image(&greyscale)
        .save("output/greyscale.png")
        .unwrap();
    render::colour_image(&colour)
        .save("output/colour.png")
        .unwrap();
}
