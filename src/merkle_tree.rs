use eyre::Result;

/// Merkle Tree representation
pub trait MerkleTree {
    /// Add a new leaf to the tree.
    fn push(&mut self, data: &[u8]) -> Result<()>;
    /// Return the root hash of the tree.
    fn root(&self) -> Result<&[u8]>;
}

pub fn new() -> Box<dyn MerkleTree> {
    Box::new(MerkleTreeImpl::new())
}

/// Merkle Tree implementation
struct MerkleTreeImpl {}
impl MerkleTreeImpl {
    fn new() -> Self {
        Self {}
    }
}

impl MerkleTree for MerkleTreeImpl {
    fn push(&mut self, _data: &[u8]) -> Result<()> {
        todo!()
    }

    fn root(&self) -> Result<&[u8]> {
        todo!()
    }
}
