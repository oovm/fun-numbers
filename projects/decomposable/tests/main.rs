use std::{fs::File, io::Write, path::Path, str::FromStr};
use super_prime::{insert_digit, super_prime, BigUint};
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

#[ignore]
#[test]
fn find_prime2() -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let mut primes = File::create(here.join("prime2.py"))?;
    let start = BigUint::from(2usize);
    for n in super_prime(&start).into_iter() {
        println!("prime2: digits {} find in {}, {} candidates left", n.digits, n.time, n.rest);
        if let Some(s) = n.numbers.last() {
            writeln!(primes, "{}", s.to_string())?;
        }
    }
    Ok(())
}
#[ignore]
#[test]
fn find_prime3() -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let mut primes = File::create(here.join("prime3.py"))?;
    let start = BigUint::from(3usize);
    for n in super_prime(&start).into_iter() {
        println!("prime3: digits {} find in {}, {} candidates left", n.digits, n.time, n.rest);
        if let Some(s) = n.numbers.last() {
            writeln!(primes, "{}", s.to_string())?;
        }
    }
    Ok(())
}

#[test]
fn find_prime5() -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let mut primes = File::create(here.join("prime5.py"))?;
    let start = BigUint::from(5usize);
    for n in super_prime(&start).into_iter() {
        println!("prime5: digits {} find in {}, {} candidates left", n.digits, n.time, n.rest);
        if let Some(s) = n.numbers.last() {
            writeln!(primes, "{}", s.to_string())?;
        }
    }
    Ok(())
}
#[ignore]
#[test]
fn find_prime7() -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let mut primes = File::create(here.join("prime7.py"))?;
    let start = BigUint::from(7usize);
    for n in super_prime(&start).into_iter() {
        println!("prime7: digits {} find in {}, {} candidates left", n.digits, n.time, n.rest);
        if let Some(s) = n.numbers.last() {
            writeln!(primes, "{}", s.to_string())?;
        }
    }
    Ok(())
}
