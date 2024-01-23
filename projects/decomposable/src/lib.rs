#![feature(impl_trait_in_assoc_type)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

pub use crate::{
    generators::DigitsGenerator,
    pair::{DecomposablePair, DecomposablePairItem},
};
use std::{fs::File, io::Write};
mod generators;
mod pair;

#[test]
fn main() -> std::io::Result<()> {
    let mut output = File::create("output.tex")?;
    for digit in 1..=3 {
        output.write_all(b"$$\n")?;
        write_digits(&mut output, digit)?;
        output.write_all(b"$$\n")?;
    }
    Ok(())
}

fn write_digits(output: &mut File, digit: u32) -> std::io::Result<()> {
    let gen = DigitsGenerator::default();
    output.write_all(b"\\begin{aligned}\n")?;
    for digits in gen.take(10usize.pow(digit + 1)).skip(10usize.pow(digit)) {
        match DecomposablePair::new(&digits) {
            Some(decomposable) => {
                output.write_fmt(format_args!("{:#}", decomposable))?;
            }
            None => {}
        }
    }
    output.write_all(b"\\end{aligned}\n")
}
