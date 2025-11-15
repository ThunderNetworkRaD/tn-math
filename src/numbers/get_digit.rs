use super::Number;

pub fn get_digit(integer_part: &Vec<u8>, rational_part: &Vec<u8>, index: usize, from_end: bool) -> u8 {
    let integer_len = integer_part.len();
    let rational_len = rational_part.len();

    if !from_end {
        if index < integer_len {
            // Parte intera è MSB-first → ribaltiamo
            return integer_part[integer_len - 1 - index];
        } else {
            // Parte razionale è LSB-first → accesso diretto
            return rational_part[index - integer_len];
        }
    } else {
        if index < rational_len {
            // Parte razionale è MSB-first → ribaltiamo
            rational_part[rational_len - 1 - index]
        } else {
            // Parte intera è LSB-first → accesso diretto
            integer_part[index - rational_len]
        }
    }
}

impl Number {
    pub fn positive_get_digit(&self, index: usize, from_end: bool) -> u8 {
        get_digit(&self.integer_part, &self.rational_part, index, from_end)
    }
}