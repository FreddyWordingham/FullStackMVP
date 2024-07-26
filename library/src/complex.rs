use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul},
};

/// Complex number representation.
#[derive(Clone, Copy)]
pub struct Complex {
    pub re: f64, // Real component.
    pub im: f64, // Imaginary component.
}

impl Complex {
    /// Construct a complex number with real and imaginary component equal to zero.
    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    /// Construct a new complex number from the given real and imaginary components.
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Compute the normal squared of the complex number.
    pub fn norm_squared(self) -> f64 {
        (self.re * self.re) + (self.im * self.im)
    }

    /// Compute the normal of the complex number.
    pub fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }
}

impl Add for Complex {
    type Output = Self;

    /// Add two complex numbers together.
    fn add(self, complex_rhs: Self) -> Self {
        Self {
            re: self.re + complex_rhs.re,
            im: self.im + complex_rhs.im,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Self;

    /// Multiply this complex number by a given scalar.
    fn mul(self, scalar_rhs: f64) -> Self {
        Self {
            re: self.re * scalar_rhs,
            im: self.re * scalar_rhs,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    /// Multiply two complex numbers together.
    fn mul(self, complex_rhs: Self) -> Self {
        Self {
            re: (self.re * complex_rhs.re) - (self.im * complex_rhs.im),
            im: (self.re * complex_rhs.im) + (self.im * complex_rhs.re),
        }
    }
}

impl Display for Complex {
    /// Write a string representation of the complex number to the given formatter.
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.re, self.im)
    }
}
