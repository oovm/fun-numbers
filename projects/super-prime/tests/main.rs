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

#[test]
fn test() -> std::io::Result<()> {
    const LIMIT: usize = 100;
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").canonicalize()?;
    let mut prime2 = File::create(here.join("prime2.py"))?;
    let mut prime3 = File::create(here.join("prime3.py"))?;
    let mut prime5 = File::create(here.join("prime5.py"))?;
    let mut prime7 = File::create(here.join("prime7.py"))?;
    let start = BigUint::from(2usize);
    for n in super_prime(&start, LIMIT).into_iter() {
        writeln!(prime2, "{}", n.to_string())?;
    }
    let start = BigUint::from(3usize);
    for n in super_prime(&start, LIMIT).into_iter() {
        writeln!(prime3, "{}", n.to_string())?;
    }
    let start = BigUint::from(5usize);
    for n in super_prime(&start, LIMIT).into_iter() {
        writeln!(prime5, "{}", n.to_string())?;
    }
    let start = BigUint::from(7usize);
    for n in super_prime(&start, LIMIT).into_iter() {
        writeln!(prime7, "{}", n.to_string())?;
    }
    Ok(())
}
