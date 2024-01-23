#[derive(Debug)]
pub struct DigitsGenerator {
    digits: Vec<u8>,
}

impl Default for DigitsGenerator {
    fn default() -> Self {
        Self { digits: vec![0] }
    }
}

impl<'i> Iterator for DigitsGenerator {
    type Item = Vec<u8>;
    // [] -> [0] -> [1] -> ... [9] -> [1,0] -> [1,1] -> ...
    fn next(&mut self) -> Option<Self::Item> {
        if self.digits.is_empty() {
            self.digits.push(0);
            return Some(self.digits.clone());
        }
        let mut carry = true;
        for digit in self.digits.iter_mut() {
            if carry {
                *digit += 1;
                carry = *digit == 10;
                if carry {
                    *digit = 0;
                }
            }
        }
        if carry {
            self.digits.push(1);
        }
        Some(self.digits.iter().cloned().rev().collect())
    }
}
