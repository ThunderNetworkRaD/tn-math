use super::{Number, Sign};

pub trait NumberTrait {
    fn digits(&self) -> Vec<u8>;
    fn to_number(&self) -> Number;
}

impl NumberTrait for usize {
    /// Returns a vector of digits representing the number.
    ///
    /// The vector contains the digits of the number in the order of
    /// least significant digit first (i.e. right to left).
    ///
    /// For example, the number 123 would be returned as the vector
    /// [3, 2, 1].
    fn digits(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        let mut n = *self;
        while n > 0 {
            v.push((n % 10).try_into().unwrap());
            n /= 10;
        }
        v
    }

    /// Converts the number into a `Number` struct.
    ///
    /// The resulting `Number` struct has the digits of the number in the
    /// `integer_part` field, an empty `rational_part` field, and a
    /// `Sign` of `Sign::Positive`.
    fn to_number(&self) -> Number {
        let mut number = Number {
            integer_part: self.digits(),
            rational_part: Vec::new(),
            sign: Sign::Positive,
        };
        number.minify();
        number
    }
}