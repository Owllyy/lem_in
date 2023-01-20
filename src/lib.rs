//! This is a 42 school project
//! The subject can be found [here](https://cdn.intra.42.fr/pdf/pdf/63626/en.subject.pdf)
#![allow(soft_unstable)]
#![feature(test)]

#[cfg(test)]
macro_rules! include_str_abs {
    ($path: literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), $path))
    }
}

pub mod graph;
pub mod bit_array;

pub use graph::*;
pub use bit_array::BitArray;