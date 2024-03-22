use std::vec;

use crate::{
    codec::{self, Codec},
    Matrix2D, Matrix3D,
};
use eyre::{eyre, Result};

/// Store all data for an original data square (ODS) or extended data square (EDS).
/// Data is duplicated in both row-major and column-major order in order to be able to provide zero-allocation column slices.
pub struct DataSquare {
    pub square_row: Matrix3D,
    pub square_col: Matrix3D,
    pub width: usize,
    pub original_width: usize,
    pub chunk_size: usize,
    pub row_roots: Matrix2D,
    pub col_roots: Matrix2D,
    pub codec: Box<dyn Codec>,
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
    pub fn new(original_data: Matrix2D) -> Result<Self> {
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

        let codec = codec::new(original_data.len())?;

        Ok(Self {
            square_row,
            square_col,
            width,
            original_width: width,
            chunk_size,
            row_roots: vec![],
            col_roots: vec![],
            codec,
        })
    }

    /// Extend original square horizontally and vertically
    ///  ------- -------
    /// |       |       |
    /// |   O → |   E   |
    /// |   ↓   |       |
    ///  ------- -------
    /// |       |
    /// |   E   |
    /// |       |
    ///  -------
    pub fn erasure_extend_square(&mut self) -> Result<()> {
        self.original_width = self.width;
        // Extend data square.
        self.extend_square()?;

        for i in 0..self.original_width {
            // Extend horizontally with Reed Solomon encoding.
            self.erasure_extend_row(i)?;
            // Extend vertically with Reed Solomon encoding.
            self.erasure_extend_col(i)?;
        }

        Ok(())
    }

    /// Extend original square horizontally and vertically
    ///
    /// # Returns
    /// * `Result` - Error.
    fn extend_square(&mut self) -> Result<()> {
        self.width *= 2;
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
        self.square_col = vec![vec![]; self.width];
        for (j, col) in self.square_col.iter_mut().enumerate().take(self.width) {
            for i in 0..self.width {
                col.push(self.square_row[i][j].to_vec());
            }
        }

        Ok(())
    }

    /// Encode a given row.
    ///
    /// # Arguments
    ///
    /// * `row_index` - The row index.
    ///
    /// # Returns
    ///
    /// * `()` - Unit.
    /// * `Result` - Error.
    fn erasure_extend_row(&mut self, row_index: usize) -> Result<()> {
        // Create a new codec.
        // TODO: implement cache for codec.
        let codec = codec::new(self.original_width)?;
        // Apply Reed-Solomon encoding.
        let shares = self.square_row[row_index][0..self.original_width].to_vec();
        let encoded_shares = codec.encode(shares)?;
        // Save encoded row in square row.
        for i in 0..encoded_shares.len() {
            self.square_row[row_index][self.original_width + i] = encoded_shares[i].clone();
            self.square_col[self.original_width + i][row_index] = encoded_shares[i].clone();
        }
        Ok(())
    }

    /// Encode a given col.
    ///
    /// # Arguments
    ///
    /// * `i` - The col index.
    ///
    /// # Returns
    ///
    /// * `()` - Unit.
    /// * `Result` - Error.
    fn erasure_extend_col(&mut self, col_index: usize) -> Result<()> {
        // Create a new codec.
        // TODO: implement cache for codec.
        let codec = codec::new(self.original_width)?;
        // Apply Reed-Solomon encoding.
        let shares = self.square_col[col_index][0..self.original_width].to_vec();

        let encoded_shares = codec.encode(shares)?;
        // Save encoded row in square row.
        for i in 0..encoded_shares.len() {
            self.square_row[self.original_width + i][col_index] = encoded_shares[i].clone();
            self.square_col[col_index][self.original_width + i] = encoded_shares[i].clone();
        }
        Ok(())
    }
}

/// Represent an extended piece of data.
pub struct ExtendedDataSquare {
    pub data_square: DataSquare,
}
