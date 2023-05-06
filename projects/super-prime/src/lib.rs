#![feature(iter_from_generator)]
#![feature(generators)]

use std::collections::VecDeque;
use std::iter::from_generator;
use std::ops::{Div, Mul, Rem};
use std::str::FromStr;
use num_primes::{Verification, BigUint};


/// insert 0-9 to n at given position
pub fn insert_digit(n: &BigUint, position: usize, contains_zero: bool) -> impl Iterator<Item=BigUint> + '_ {
    let start_index = if contains_zero {
        0
    } else {
        1
    };
    from_generator(move || {
        for i in start_index..=9 {
            let pow = 10usize.pow(position as u32);
            let lhs = n.div(&pow);
            let rhs = n.rem(&pow);
            let new = lhs.mul(&pow * 10) + i.mul(&pow) + rhs;
            if Verification::is_prime(&new) {
                yield new;
            }
        }
    })
}

/// find a sequence, starting from a decimal one-digit prime number, insert a number at any position, it is still a prime number
/// eg, 1 -> 13 -> 137 -> 1373 -> ...
pub fn find_dfs(start: &BigUint, max_length: usize) -> Vec<BigUint> {
    let start_len = start.to_string().len();
    assert!(start_len < max_length, "start number {} must be less than max length {}", start, max_length);
    // dfs search, return first stack reached max length
    let seq = vec![start.clone()];
    let mut stack = VecDeque::new();
    stack.push_back(seq);
    'outer: while let Some(old_seq) = stack.pop_back() {
        let old_len = old_seq.len() + start_len - 1;
        let last = old_seq.last().unwrap();
        for i in 0..old_seq.len() {
            for number in insert_digit(last, i, i != old_len) {
                let mut new_seq = old_seq.clone();
                new_seq.push(number.clone());
                stack.push_back(new_seq);
                let new_len = old_len + 1;
                if new_len == max_length {
                    break 'outer;
                }
            }
        }
    }
    stack.pop_back().unwrap()
}

#[test]
fn test() {
    let start = BigUint::from_str("1391597082670536199999").unwrap();
    for n in find_dfs(&start, 2) {
        println!("{}", n);
    }
}