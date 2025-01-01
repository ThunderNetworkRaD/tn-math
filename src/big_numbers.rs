#[derive(Debug, Clone)]
pub struct BigDecimal {
    integer_part: Vec<u8>,
    fractional_part: Vec<u8>,
}

impl BigDecimal {
    pub fn new(value: f64, precision: usize) -> Self {
        let integer_part = (value.trunc() as u64)
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();

        let mut fractional_part = Vec::new();
        let mut fractional_value = value.fract();

        for _ in 0..precision {
            fractional_value *= 10.0;
            let digit = fractional_value.trunc() as u8;
            fractional_part.push(digit);
            fractional_value -= fractional_value.trunc();
        }

        BigDecimal {
            integer_part,
            fractional_part,
        }
    }

    pub fn from_str(s: &str, precision: usize) -> Result<Self, String> {
        let mut parts = s.split('.');
        let integer_part_str = parts.next().unwrap_or("");
        let fractional_part_str = parts.next().unwrap_or("");

        let integer_part = integer_part_str
            .chars()
            .map(|c| c.to_digit(10).ok_or("Non è un numero valido"))
            .collect::<Result<Vec<u8>, &str>>()?;

        let mut fractional_part = Vec::new();
        for (i, c) in fractional_part_str.chars().enumerate() {
            if i >= precision {
                break;
            }
            let digit = c.to_digit(10).ok_or("Non è un numero valido")?;
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

    pub fn add(&self, other: &BigDecimal) -> BigDecimal {
        let mut result_integer = Vec::new();
        let mut result_fractional = Vec::new();
        let mut carry = 0;

        let max_fractional_len = self.fractional_part.len().max(other.fractional_part.len());
        let mut self_frac = self.fractional_part.clone();
        let mut other_frac = other.fractional_part.clone();

        while self_frac.len() < max_fractional_len {
            self_frac.push(0);
        }
        while other_frac.len() < max_fractional_len {
            other_frac.push(0);
        }

        for i in (0..max_fractional_len).rev() {
            let sum = self_frac[i] + other_frac[i] + carry;
            result_fractional.push(sum % 10);
            carry = sum / 10;
        }

        result_fractional.reverse();

        let max_integer_len = self.integer_part.len().max(other.integer_part.len());
        let mut self_int = self.integer_part.clone();
        let mut other_int = other.integer_part.clone();

        while self_int.len() < max_integer_len {
            self_int.insert(0, 0);
        }
        while other_int.len() < max_integer_len {
            other_int.insert(0, 0);
        }

        for i in (0..max_integer_len).rev() {
            let sum = self_int[i] + other_int[i] + carry;
            result_integer.push(sum % 10);
            carry = sum / 10;
        }

        result_integer.reverse();

        BigDecimal {
            integer_part: result_integer,
            fractional_part: result_fractional,
        }
    }

    pub fn multiply(&self, other: &BigDecimal) -> BigDecimal {
        let mut result_integer = vec![0u8; self.integer_part.len() + other.integer_part.len()];
        let mut result_fractional = vec![0u8; self.fractional_part.len() + other.fractional_part.len()];

        for i in (0..self.integer_part.len()).rev() {
            let mut carry = 0;
            for j in (0..other.integer_part.len()).rev() {
                let mul = self.integer_part[i] * other.integer_part[j] + result_integer[i + j + 1] + carry;
                result_integer[i + j + 1] = mul % 10;
                carry = mul / 10;
            }
            result_integer[i] = carry;
        }

        for i in (0..self.fractional_part.len()).rev() {
            let mut carry = 0;
            for j in (0..other.fractional_part.len()).rev() {
                let mul = self.fractional_part[i] * other.fractional_part[j] + result_fractional[i + j + 1] + carry;
                result_fractional[i + j + 1] = mul % 10;
                carry = mul / 10;
            }
            result_fractional[i] = carry;
        }

        BigDecimal {
            integer_part: result_integer,
            fractional_part: result_fractional,
        }
    }
}