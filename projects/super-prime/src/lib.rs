#![feature(iter_from_generator)]
#![feature(generators)]

use std::collections::VecDeque;
use std::iter::from_generator;
use std::ops::{Div, Mul, Rem};
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
pub fn find_dfs(start: &BigUint, max_length: usize) -> VecDeque<BigUint> {
    let start_len = start.to_string().len();
    assert!(start_len < max_length, "length of {} is larger than {}", start, max_length);
    // dfs search, return first stack reached max length
    let mut stack = VecDeque::new();
    stack.push_back(start.clone());
    'outer: while let Some(n) = stack.pop_back() {
        let old_len = n.to_string().len();
        for i in 0..old_len {
            for new in insert_digit(&n, i, i != old_len) {
                let new_len = old_len + 1;
                stack.push_back(new.clone());
                if new_len == max_length {
                    break 'outer;
                }
            }
        }
    }
    stack
}

#[test]
fn test() {
    let start = BigUint::from(13usize);
    for n in find_dfs(&start, 20) {
        println!("{}", n);
    }
}