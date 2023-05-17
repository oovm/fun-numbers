#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![feature(generators, iter_from_generator)]

mod dfs;

pub use crate::dfs::{insert_digit, super_prime};
pub use num::BigUint;
pub use num_prime::{nt_funcs::is_prime, PrimalityTestConfig};

/// A super prime record
#[derive(Debug, Clone)]
pub struct SuperPrimeRecord {
    /// The super prime number
    pub numbers: Vec<BigUint>,
    /// The number of digits of the super prime number
    pub digits: usize,
    /// The number of digits of the super prime number
    pub rest: usize,
    /// The time spent on finding the super prime number
    pub time: f64,
}
