use crate::{merkle_tree::MerkleTree, Matrix2D, Matrix3D};
use eyre::{eyre, Result};

/// Store all data for an original data square (ODS) or extended data square (EDS).
/// Data is duplicated in both row-major and column-major order in order to be able to provide zero-allocation column slices.
#[derive(Debug)]
pub struct DataSquare {
    pub square_row: Matrix3D,
    pub square_col: Matrix3D,
    pub width: usize,
    pub chunk_size: usize,
    pub row_roots: Matrix2D,
    pub col_roots: Matrix2D,
}

impl DataSquare {
    pub fn new(original_data: Matrix2D, _merkle_tree: &dyn MerkleTree) -> Result<Self> {
        let data_len = f64::from(original_data.len() as u32);
        let width = data_len.sqrt();
        if width * width != data_len {
            return Err(eyre!("Number of chunks must be a square number"));
        }
        Ok(Self {
            square_row: Matrix3D::default(),
            square_col: Matrix3D::default(),
            width: width as usize,
            chunk_size: 0,
            row_roots: vec![vec![0; original_data[0].len()]; original_data.len()],
            col_roots: vec![vec![0; original_data[0].len()]; original_data.len()],
        })
    }
}

/// Represent an extended piece of data.
pub struct ExtendedDataSquare {
    pub data_square: DataSquare,
}
