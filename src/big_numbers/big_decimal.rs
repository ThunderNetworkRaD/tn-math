mod sum;
mod sanitize;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BigDecimal {
    pub integer_part: Vec<u8>,
    pub fractional_part: Vec<u8>,
}

impl BigDecimal {
    pub fn from_str(s: &str) -> Result<Self, String> {
        let mut parts = s.split('.');
        let integer_part_str = parts.next().unwrap_or("");
        let fractional_part_str = parts.next().unwrap_or("");

        let integer_part = integer_part_str
            .chars()
            .map(|c| c.to_digit(10).expect("That's not a valid number (integer part)") as u8)
            .collect::<Vec<u8>>();

        let mut fractional_part = Vec::new();
        for (_i, c) in fractional_part_str.chars().enumerate() {
            let digit = c.to_digit(10).ok_or("That's not a valid number (fractional part)")?;
            fractional_part.push(digit as u8);
        }

        Ok(BigDecimal {
            integer_part,
            fractional_part,
        })
    }

    pub fn to_string(&self) -> String {
        let integer_str: String = self
            .integer_part
            .iter()
            .map(|&d| (b'0' + d) as char)
            .collect();

        let fractional_str: String = self
            .fractional_part
            .iter()
            .map(|&d| (b'0' + d) as char)
            .collect();

        if !self.fractional_part.is_empty() {
            format!("{}.{}", integer_str, fractional_str)
        } else {
            integer_str
        }
    }
}