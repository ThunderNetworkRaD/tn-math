use std::fmt::Debug;
use super::{Number, Sign};

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign_str = match self.sign {
            Sign::Positive => "",
            Sign::Negative => "-",
        };

        let integer_str: String = self.integer_part.iter().rev().map(|d| d.to_string()).collect();
        let rational_str: String = self.rational_part.iter().map(|d| d.to_string()).collect();

        if self.rational_part.is_empty() {
            write!(f, "{}{}", sign_str, integer_str)
        } else {
            write!(f, "{}{}.{}", sign_str, integer_str, rational_str)
        }
    }
}