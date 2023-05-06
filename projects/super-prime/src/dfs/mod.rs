use std::collections::VecDeque;
use std::iter::from_generator;
use std::ops::{Div, Mul, Rem};
use num_primes::{Verification, BigUint};
use num_traits::Pow;


/// insert 0-9 to n at given position
pub fn insert_digit(n: &BigUint, position: usize, contains_zero: bool) -> impl Iterator<Item=BigUint> + '_ {
    let start_index: u8 = if contains_zero {
        0
    } else {
        1
    };
    let pow = BigUint::from(10usize).pow(position);
    let lhs = n.div(&pow).mul(&pow).mul(10usize);
    let rhs = n.rem(&pow);
    from_generator(move || {
        for i in start_index..=9 {
            let digit = BigUint::from(i).mul(&pow);
            let new = digit + &lhs + &rhs;
            if Verification::is_prime(&new) {
                yield new;
            }
        }
    })
}

/// A super prime is a prime number that, when any of its digits is deleted, the remaining number is still prime.
///
/// This function returns a sequence of super primes, starting from a given number, with a given length.
///
/// eg:
/// - 1 -> 19 -> 199 -> 1999 -> 13999 -> ...
/// - 2 -> 29 -> 269 -> 2969 -> 25969 -> ...
///
/// ## Examples
///
/// ```
/// # use super_prime::{BigUint, super_prime};
///     let start = BigUint::from(2usize);
///     for n in super_prime(&start, 100).into_iter().rev() {
///         println!("{}", n);
///     }
/// ```
pub fn super_prime(start: &BigUint, iteration: usize) -> Vec<BigUint> {
    let seq = vec![start.clone()];
    let mut stack = VecDeque::new();
    stack.push_back(seq);
    // dfs search, return first stack reached max length
    'outer: while let Some(old_seq) = stack.pop_back() {
        let last = unsafe {
            old_seq.last().unwrap_unchecked()
        };
        let old_len = last.to_string().len();
        for i in 0..old_len {
            for number in insert_digit(last, i, i != old_len) {
                let mut new_seq = old_seq.clone();
                new_seq.push(number.clone());
                stack.push_back(new_seq);
                if old_len == iteration {
                    break 'outer;
                }
            }
        }
    }
    // nil means no such sequence
    stack.pop_back().unwrap_or_default()
}

