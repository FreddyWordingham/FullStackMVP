mod complex;
mod cpu;
mod render;

pub use complex::Complex;
pub use cpu::{multi_sample, sample_area, sample_point};
pub use render::{render_colour_image, render_greyscale_image};

// use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
pub fn my_pure_function(a: usize, b: usize) -> String {
    (a + b).to_string()
}

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn my_rust_function(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// /// A Python module implemented in Rust. The name of this function must match
// /// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
// /// import the module.
// #[pymodule]
// fn my_library(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(my_rust_function, m)?)?;
//     Ok(())
// }
