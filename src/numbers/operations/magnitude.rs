use super::super::Number;

impl Number {
    pub fn shift_rational_to_integer(&mut self) {
        let digit = if self.rational_part.is_empty() {
            0
        } else {
            self.rational_part.remove(0)
        };
        if digit == 0 && self.integer_part.is_empty() {
            return;
        }
        self.integer_part.insert(0, digit);
    }

    pub fn shift_rationals_to_integers(&mut self, count: usize) {
        for _ in 0..count {
            self.shift_rational_to_integer();
        }
    }

    pub fn shift_integer_to_rational(&mut self) {
        let digit = if self.integer_part.is_empty() {
            0
        } else {
            self.integer_part.remove(0)
        };
        if digit == 0 && self.rational_part.is_empty() {
            return;
        }
        self.rational_part.insert(0, digit);
    }

    pub fn shift_integers_to_rationals(&mut self, count: usize) {
        for _ in 0..count {
            self.shift_integer_to_rational();
        }
    }
}