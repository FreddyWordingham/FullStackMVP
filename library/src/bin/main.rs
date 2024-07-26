use my_library::{filter, pipeline, render, Config};

fn main() {
    // Load settings from config file
    let settings = Config::load("input/config.yml")
        .expect("Failed to read config file")
        .build();

    // Run the pipeline
    let (_samples, shadow_map, greyscale, mut colour) = pipeline::run(&settings);

    // Render images
    render::greyscale_image(&shadow_map.map(|&x| (x * 255.0) as u8))
        .save("output/shadow_map.png")
        .unwrap();

    render::greyscale_image(&greyscale)
        .save("output/greyscale.png")
        .unwrap();

    render::colour_image(&colour)
        .save("output/colour.png")
        .unwrap();

    filter::apply_shadow_map(&mut colour, &shadow_map);
    render::colour_image(&colour)
        .save("output/final.png")
        .unwrap();
}
