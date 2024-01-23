use crate::SuperPrimeRecord;
use num::BigUint;
use num_prime::nt_funcs::is_prime;
use std::{
    collections::VecDeque,
    iter::from_generator,
    ops::{Div, Mul, Rem},
};

/// Insert 0-9 to number at given position
///
/// # Examples
///
/// ```
/// # use super_prime::{insert_digit, BigUint};
/// let start = BigUint::from(13usize);
/// for n in insert_digit(&start, 0, true) {
///     println!("{}", n); // 131, 137, 139
/// }
/// for n in insert_digit(&start, 1, true) {
///     println!("{}", n); // 103, 113, 167, 173, 193
/// }
/// for n in insert_digit(&start, 2, false) {
///     println!("{}", n); // 113, 313, 613
/// }
/// ```
pub fn insert_digit(n: &BigUint, position: usize, contains_zero: bool) -> impl Iterator<Item = BigUint> + '_ {
    debug_assert!(position <= n.to_string().len(), "position {} of {} is out of range", position, n);
    let start_index: u8 = if contains_zero { 0 } else { 1 };
    let pow = BigUint::from(10usize).pow(position as u32);
    let lhs = n.div(&pow).mul(&pow).mul(10usize);
    let rhs = n.rem(&pow);
    from_generator(move || {
        for i in start_index..=9 {
            let digit = BigUint::from(i).mul(&pow);
            let new = digit + &lhs + &rhs;
            if is_prime(&new, None).probably() {
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
/// let start = BigUint::from(2usize);
/// for n in super_prime(&start).into_iter().rev() {
///     println!("{}", n);
/// }
/// ```
pub fn super_prime(start: &BigUint) -> impl Iterator<Item = SuperPrimeRecord> {
    let start_time = std::time::Instant::now();
    let mut old_len = start.to_string().len();
    let seq = vec![start.clone()];
    let mut stack = VecDeque::new();
    stack.push_back(seq);
    // pair search, return first stack reached max length
    from_generator(move || {
        while let Some(old_seq) = stack.pop_back() {
            let last = unsafe { old_seq.last().unwrap_unchecked() };
            let new_len = last.to_string().len();
            for i in 0..old_len {
                for number in insert_digit(last, i, i != old_len) {
                    let mut new_seq = old_seq.clone();
                    new_seq.push(number.clone());
                    stack.push_back(new_seq);
                }
            }
            if new_len > old_len {
                yield SuperPrimeRecord {
                    numbers: old_seq,
                    digits: old_len,
                    rest: stack.len(),
                    time: start_time.elapsed().as_secs_f64(),
                };
                old_len = new_len;
            }
        }
    })
}
