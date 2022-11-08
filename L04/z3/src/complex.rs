use std::ops::{Add, Sub, Mul};

#[derive(Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
 }

impl Complex{
    pub fn print(&self){
        println!("{} + {}i\n", self.real.to_string(), self.imaginary.to_string());
    }

    pub fn abs(&self) -> f64{
        (self.real * self.real + self.imaginary * self.imaginary).sqrt() 
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {real: self.real + other.real, imaginary: self.imaginary + other.imaginary}
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {real: self.real - other.real, imaginary: self.imaginary - other.imaginary}
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {real: self.real * other.real - self.imaginary * other.imaginary, imaginary: self.imaginary * other.real + self.real * other.imaginary}
    }
}
