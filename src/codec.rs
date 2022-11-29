use eyre::Result;
use reed_solomon_erasure::galois_8::ReedSolomon;

use crate::{Matrix2D, Matrix3D};

/// Codec handle for encoding and decoding data
pub trait Codec {
    /// Encode encodes original data, automatically extracting share size.
    /// There must be no missing shares. Only returns parity shares.
    fn encode(&self, data: Matrix2D) -> Result<Matrix2D>;
    /// Decode sparse original + parity data, automatically extracting share size.
    /// Missing shares must be nil. Returns original shares only.
    fn decode(&self, data: Matrix3D) -> Result<Matrix3D>;
    /// Return the max. number of chunks each code supports in a 2D square.
    fn max_chunks(&self) -> usize;
}

pub fn new(data_len: usize) -> Result<Box<dyn Codec>> {
    Ok(Box::new(ReedSolomonCodec::new(data_len)?))
}

const REED_SOLOMON_MAX_CHUNKS: usize = 32768 * 32768;

/// Reed Solomon Codec implementation
struct ReedSolomonCodec {
    data_len: usize,
    rs: ReedSolomon,
}

impl ReedSolomonCodec {
    /// Create a new ReedSolomonCodec instance.
    fn new(data_len: usize) -> Result<Self> {
        Ok(Self {
            data_len,
            rs: ReedSolomon::new(data_len, data_len)?,
        })
    }
}

impl Codec for ReedSolomonCodec {
    /// Encode encodes original data, automatically extracting share size.
    /// There must be no missing shares. Only returns parity shares.
    fn encode(&self, data: Matrix2D) -> Result<Matrix2D> {
        let original_data_len = data.len();
        let mut shards = vec![vec![0; self.data_len]; self.data_len];
        for (i, row) in data.iter().enumerate() {
            shards[i] = row.clone();
        }
        for _i in 0..data.len() {
            shards.push(vec![0; data[0].len()])
        }
        self.rs.encode(&mut shards)?;
        let encoded_data = shards[original_data_len..].to_vec();
        Ok(encoded_data)
    }

    /// Decode sparse original + parity data, automatically extracting share size.
    /// Missing shares must be nil. Returns original shares only.
    fn decode(&self, _data: Matrix3D) -> Result<Matrix3D> {
        todo!()
    }

    /// Return the max. number of chunks each code supports in a 2D square.
    fn max_chunks(&self) -> usize {
        REED_SOLOMON_MAX_CHUNKS
    }
}
