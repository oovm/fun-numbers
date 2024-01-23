use num::{BigUint, Zero};
use std::{
    collections::{BTreeMap, HashSet},
    fmt::{Display, Formatter},
};

mod display;

#[derive(Clone, Debug)]
pub struct DecomposablePair {
    number: BigUint,
    solutions: BTreeMap<BigUint, Vec<(BigUint, BigUint)>>,
}

impl IntoIterator for DecomposablePair {
    type Item = DecomposablePairItem;
    type IntoIter = impl Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.solutions.into_iter().map(move |(result, factors)| DecomposablePairItem::new(&self.number, &result, &factors))
    }
}

#[derive(Default, Debug)]
pub struct DecomposablePairItem {
    pub number: BigUint,
    pub result: BigUint,
    pub factors: Vec<(BigUint, BigUint)>,
    pub symmetrical: bool,
}

impl DecomposablePairItem {
    pub fn new(number: &BigUint, result: &BigUint, factors: &[(BigUint, BigUint)]) -> Self {
        let count = factors.len();
        let unique = factors.iter().map(|(lhs, rhs)| (lhs.min(rhs), lhs.max(rhs))).collect::<HashSet<_>>().len();
        Self { number: number.clone(), result: result.clone(), factors: factors.to_vec(), symmetrical: count != unique }
    }
}

impl DecomposablePair {
    /// Decompose a number into a set of factors.
    /// Eg. &[2,6,5] -> {130: [(2, 65), (26, 5)]}
    pub fn new(digits: &[u8]) -> Option<DecomposablePair> {
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
        Some(DecomposablePair { number, solutions })
    }
    pub fn strict(&self) -> bool {
        self.solutions.len() == 1
    }
}

fn build_number(digits: &[u8]) -> BigUint {
    let mut number = BigUint::zero();
    for digit in digits {
        number = number * BigUint::from(10u32) + BigUint::from(*digit);
    }
    number
}
