//! Rust implementation of two dimensional Reed-Solomon merkle tree data availability scheme.
extern crate reed_solomon_erasure;

pub mod codec;
pub mod data_square;
pub mod merkle_tree;
pub mod rsmt2d;

pub type Matrix2D = Vec<Vec<u8>>;
pub type Matrix3D = Vec<Vec<Vec<u8>>>;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
