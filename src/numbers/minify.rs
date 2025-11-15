use super::Number;

pub fn minify(vector: &mut Vec<u8>, begin: bool) {
    while vector.len() > 1 && vector[if begin { 0 } else { vector.len() - 1 }] == 0 {
        if begin {
            vector.remove(0);
        } else {
            vector.pop();
        }
    }
}

impl Number {
    pub fn minify(&mut self) {
        minify(&mut self.integer_part, false);
        minify(&mut self.rational_part, true);
    }    
}