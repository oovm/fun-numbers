use num::{BigUint, Zero};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct Decomposable {
    number: BigUint,
    solutions: BTreeMap<BigUint, Vec<(BigUint, BigUint)>>,
}

impl Decomposable {
    /// Decompose a number into a set of factors.
    /// Eg. &[2,6,5] -> {130: [(2, 65), (26, 5)]}
    pub fn new(digits: &[u8]) -> Option<Decomposable> {
        let number = build_number(digits);
        let mut solutions = BTreeMap::new();
        for i in 0..digits.len() {
            let lhs = build_number(&digits[..i]);
            let rhs = build_number(&digits[i..]);
            solutions.entry(&lhs * &rhs).or_insert_with(Vec::new).push((lhs, rhs));
        }
        solutions.retain(|k, v| !k.is_zero() && v.len() > 1);
        if solutions.is_empty() {
            return None;
        }
        Some(Decomposable { number, solutions })
    }
    pub fn strict(&self) -> bool {
        self.solutions.len() == 1
    }
}

impl Display for Decomposable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (result, factors) in self.solutions.iter() {
            write!(f, "{}", result)?;
            for (index, (lhs, rhs)) in factors.iter().enumerate() {
                if f.alternate() && index == 0 {
                    write!(f, "&={}×{}", lhs, rhs)?;
                }
                else {
                    write!(f, "={}×{}", lhs, rhs)?;
                }
            }
            f.write_str("\\\\\n")?;
        }
        Ok(())
    }
}

fn build_number(digits: &[u8]) -> BigUint {
    let mut number = BigUint::zero();
    for digit in digits {
        number = number * BigUint::from(10u32) + BigUint::from(*digit);
    }
    number
}
#[derive(Default)]
pub struct DigitsGenerator {
    digits: Vec<u8>,
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
