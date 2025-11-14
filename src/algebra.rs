pub mod addition;
pub mod subtraction;
pub mod multiplication;
pub mod division;
pub mod power;

pub trait Algebra {
    fn multiply_u8(self, other: u8) -> u8;
    fn sum_u8(self, other: u8) -> u8;
}

impl Algebra for u8 {
    fn sum_u8(self, other: u8) -> u8 {
        self + other
    }

    fn multiply_u8(self, other: u8) -> u8 {
        self * other
    }
}