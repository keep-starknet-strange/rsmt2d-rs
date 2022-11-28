use std::vec;

use crate::{merkle_tree::MerkleTree, Matrix2D, Matrix3D};
use eyre::{eyre, Result};

/// Store all data for an original data square (ODS) or extended data square (EDS).
/// Data is duplicated in both row-major and column-major order in order to be able to provide zero-allocation column slices.
#[derive(Debug)]
pub struct DataSquare {
    pub square_row: Matrix3D,
    pub square_col: Matrix3D,
    pub width: usize,
    pub original_width: usize,
    pub chunk_size: usize,
    pub row_roots: Matrix2D,
    pub col_roots: Matrix2D,
}

impl DataSquare {
    /// Create a new data square.
    /// Data must be a square matrix.
    /// Data must not be empty.
    ///
    /// # Arguments
    ///
    /// * `original_data` - Original data.
    /// * `merkle_tree` - `MerkleTree`.
    ///
    /// # Returns
    ///
    /// * `DataSquare` - Data square.
    /// * `Result` - Error.
    pub fn new(original_data: Matrix2D, _merkle_tree: &dyn MerkleTree) -> Result<Self> {
        // Check that data is not empty.
        if original_data.is_empty() {
            return Err(eyre!("Data must not be empty."));
        }

        // Check that data is a square matrix.
        let data_len = f64::from(original_data.len() as u32);
        let width = data_len.sqrt();
        if width * width != data_len {
            return Err(eyre!("Number of chunks must be a square number."));
        }
        let width: usize = width as usize;

        // Check that chunks are all the same size.
        let chunk_size = original_data[0].len();
        for row in original_data.iter() {
            if row.len() != chunk_size {
                return Err(eyre!("All chunks must be of equal size."));
            }
        }

        // Create row-major data squares.
        let mut square_row: Matrix3D = vec![vec![]; width];
        for i in 0..width {
            square_row[i] = original_data[i * width..i * width + width].to_vec();
        }

        // Create column-major data squares.
        let mut square_col: Matrix3D = vec![vec![]; width];
        for j in 0..width {
            for i in 0..width {
                square_col[j].push(original_data[i * width + j].to_vec());
            }
        }

        Ok(Self {
            square_row,
            square_col,
            width,
            original_width: width,
            chunk_size,
            row_roots: vec![],
            col_roots: vec![],
        })
    }

    /// TODO: Comment function
    pub fn erasure_extend_square(&mut self) -> Result<()> {
        self.width = 2 * self.width;
        let filler_chunk = vec![Default::default(); self.chunk_size];
        let mut filler_row: Vec<Vec<u8>> = vec![];
        for _ in 0..self.width {
            filler_row.push(filler_chunk.clone());
        }
        // Extend rows.
        for row in self.square_row.iter_mut() {
            for _ in 0..self.original_width {
                row.push(filler_chunk.clone());
            }
        }

        for i in self.original_width..self.width {
            self.square_row.insert(i, filler_row.clone());
        }
        // Extend cols.
        // let mut new_col: Matrix3D = vec![vec![Default::default(); self.width]; self.width];
        // for i in 0..self.width {
        //     for j in 0..self.width {
        //         new_col[i][j].push(self.square_row[j][i]);
        //     }
        // }

        // Extend original square horizontally and vertically
        //  ------- -------
        // |       |       |
        // |   O → |   E   |
        // |   ↓   |       |
        //  ------- -------
        // |       |
        // |   E   |
        // |       |
        //  -------
        Ok(())
    }
}

/// Represent an extended piece of data.
pub struct ExtendedDataSquare {
    pub data_square: DataSquare,
}
