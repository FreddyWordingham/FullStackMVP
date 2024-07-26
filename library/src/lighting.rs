use nalgebra::Vector3;
use ndarray::Array2;

pub fn calculate_light_map(heightmap: &Array2<f64>, light_direction: [f64; 3]) -> Array2<f64> {
    let mut light_map = Array2::zeros(heightmap.dim());

    let (height, width) = heightmap.dim();
    let light_vec =
        Vector3::new(light_direction[0], light_direction[1], light_direction[2]).normalize();

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let dzdx = (heightmap[[y, x + 1]] - heightmap[[y, x - 1]]) / 2.0;
            let dzdy = (heightmap[[y + 1, x]] - heightmap[[y - 1, x]]) / 2.0;
            let normal = Vector3::new(-dzdx, -dzdy, 1.0).normalize();
            let intensity = 1.0 - normal.dot(&light_vec).max(0.0);
            light_map[[y, x]] = intensity;
        }
    }

    for x in 0..width {
        light_map[[0, x]] = light_map[[1, x]];
        light_map[[height - 1, x]] = light_map[[height - 2, x]];
    }
    for y in 0..height {
        light_map[[y, 0]] = light_map[[y, 1]];
        light_map[[y, width - 1]] = light_map[[y, width - 2]];
    }

    light_map
}
