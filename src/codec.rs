use eyre::Result;
use reed_solomon_erasure::{galois_8::ReedSolomon};

use crate::data_square::DataSquare;

/// Codec handle for encoding and decoding data
pub trait Codec {
    /// Encode encodes original data, automatically extracting share size.
    /// There must be no missing shares. Only returns parity shares.
    fn encode(&self, data: DataSquare) -> Result<DataSquare>;
    /// Decode sparse original + parity data, automatically extracting share size.
    /// Missing shares must be nil. Returns original shares only.
    fn decode(&self, data: DataSquare) -> Result<DataSquare>;
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
    fn encode(&self, _data: DataSquare) -> Result<DataSquare> {
        todo!()
    }

    /// Decode sparse original + parity data, automatically extracting share size.
    /// Missing shares must be nil. Returns original shares only.
    fn decode(&self, _data: DataSquare) -> Result<DataSquare> {
        todo!()
    }

    /// Return the max. number of chunks each code supports in a 2D square.
    fn max_chunks(&self) -> usize {
        REED_SOLOMON_MAX_CHUNKS
    }
}
