use ndarray::Array2;

pub fn normalise(data: &mut Array2<f64>) {
    let min = data.iter().fold(0.0, |acc: f64, &x| acc.min(x));
    let max = data.iter().fold(0.0, |acc: f64, &x| acc.max(x));
    data.par_mapv_inplace(|c| (c - min) / (max - min));
}
