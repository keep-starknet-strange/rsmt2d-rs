use eyre::Result;

use crate::{
    codec::Codec,
    data_square::{ExtendedDataSquare},
    merkle_tree::MerkleTree,
    Matrix2D,
};

/// Reed Solomon Merkle Tree 2D
pub trait ReedSolomonMerkleTree2D {
    /// Compute the extended data square for some chunks of data.
    fn compute_extended_data_square(
        &self,
        data: Matrix2D,
        codec: &dyn Codec,
    ) -> Result<ExtendedDataSquare>;
    /// Import an extended data square, represented as flattened chunks of data.
    fn import_extended_data_square(
        &self,
        data: Matrix2D,
        codec: &dyn Codec,
    ) -> Result<ExtendedDataSquare>;
    /// Attempt to repair an incomplete extended data square.
    fn repair(&self, data: ExtendedDataSquare) -> Result<()>;
}

pub fn new(merkle_tree: Box<dyn MerkleTree>) -> Box<dyn ReedSolomonMerkleTree2D> {
    Box::new(ReedSolomonMerkleTree2DImpl::new(merkle_tree))
}

/// Reed Solomon Merkle Tree 2D implementation
struct ReedSolomonMerkleTree2DImpl {
    merkle_tree: Box<dyn MerkleTree>,
}

impl ReedSolomonMerkleTree2DImpl {
    /// Create a new ReedSolomonMerkleTree2DImpl instance.
    fn new(merkle_tree: Box<dyn MerkleTree>) -> Self {
        Self { merkle_tree }
    }
}

impl ReedSolomonMerkleTree2D for ReedSolomonMerkleTree2DImpl {
    fn compute_extended_data_square(
        &self,
        _data: Matrix2D,
        _codec: &dyn Codec,
    ) -> Result<ExtendedDataSquare> {
        todo!()
    }

    fn import_extended_data_square(
        &self,
        _data: Matrix2D,
        _codec: &dyn Codec,
    ) -> Result<ExtendedDataSquare> {
        todo!()
    }

    fn repair(&self, _data: ExtendedDataSquare) -> Result<()> {
        todo!()
    }
}
