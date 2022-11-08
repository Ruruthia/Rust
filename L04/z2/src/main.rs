use std::ops::{Add, Sub, Mul};

#[derive(Clone, Copy)]
struct Complex {
    real: i32,
    imaginary: i32,
 }

impl Complex{
    fn print(&self){
        println!("{} + {}i\n", self.real.to_string(), self.imaginary.to_string());
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


fn main(){
    let cmpl_1 = Complex{real:2, imaginary: 3};
    let cmpl_2 = Complex{real:-1, imaginary: 2};
    cmpl_1.print();
    cmpl_2.print();
    (cmpl_1 + cmpl_2).print();
    (cmpl_1 - cmpl_2).print();
    (cmpl_1 * cmpl_2).print();
}