use crate::{merkle_tree::MerkleTree, Matrix2D, Matrix3D};
use eyre::{eyre, Result};

/// Store all data for an original data square (ODS) or extended data square (EDS).
/// Data is duplicated in both row-major and column-major order in order to be able to provide zero-allocation column slices.
pub struct DataSquare<'a> {
    pub square_row: &'a Matrix3D<'a>,
    pub square_col: &'a Matrix3D<'a>,
    pub width: usize,
    pub chunk_size: usize,
    pub row_roots: &'a Matrix2D<'a>,
    pub col_roots: &'a Matrix2D<'a>,
}

impl<'a> DataSquare<'a> {
    pub fn new(original_data: Matrix2D, merkle_tree: &dyn MerkleTree) -> Result<Self> {
        let data_len = f64::from(original_data.len() as u32);
        let width = data_len.sqrt();
        if width * width != data_len {
            return Err(eyre!("Number of chunks must be a square number"));
        }
        todo!()
    }
}

/// Represent an extended piece of data.
pub struct ExtendedDataSquare<'a> {
    pub data_square: &'a DataSquare<'a>,
}
