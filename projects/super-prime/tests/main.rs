use std::str::FromStr;
use super_prime::{BigUint, insert_digit, super_prime};

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test2() {
    let start = BigUint::from_str("13").unwrap();
    for n in insert_digit(&start, 0, true) {
        println!("{}", n);
    }
    for n in insert_digit(&start, 1, true) {
        println!("{}", n);
    }
    for n in insert_digit(&start, 2, false) {
        println!("{}", n);
    }
}

#[test]
fn test() {
    let start = BigUint::from(2usize);
    for n in super_prime(&start, 10).into_iter().rev() {
        println!("{}", n);
    }
}