use enterpolation::{linear::Linear, Equidistant, Generator, Identity};
use palette::LinSrgb;

type ColourMap = Linear<Equidistant<f32>, Vec<LinSrgb>, Identity>;

pub struct Gradient {
    pub cmap: ColourMap,
}

impl Gradient {
    pub fn new(colours: &[&str]) -> Self {
        Self {
            cmap: Self::build_cmap(colours),
        }
    }

    fn build_cmap(colours: &[&str]) -> ColourMap {
        Linear::builder()
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
            .unwrap()
    }

    pub fn sample(&self, x: f32) -> LinSrgb {
        self.cmap.gen(x)
    }
}
