use cid::Cid;
use multihash_codetable::{Code, MultihashDigest};
use thiserror::Error;

/// An IPLD block identified by its [`Cid`].
#[derive(Clone, Debug)]
pub struct Block {
    cid: Cid,
    data: Vec<u8>,
}

impl Block {
    /// Creates a block, verifying the data hash matches the cid.
    pub fn new(cid: Cid, data: Vec<u8>) -> Result<Self, InvalidMultihash> {
        let code = Code::try_from(cid.hash().code()).map_err(|_| InvalidMultihash)?;
        let computed = code.digest(&data);
        if computed.digest() != cid.hash().digest() {
            return Err(InvalidMultihash);
        }
        Ok(Self::new_unchecked(cid, data))
    }

    /// Creates a block without verifying the hash.
    pub fn new_unchecked(cid: Cid, data: Vec<u8>) -> Self {
        Self { cid, data }
    }

    /// Returns the block's [`Cid`].
    pub fn cid(&self) -> &Cid {
        &self.cid
    }

    /// Returns the block's raw data.
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

/// Returned by [`Block::new`] when the supplied data does not hash to the supplied cid or
/// the multihash code is not in the shipped codetable.
#[derive(Debug, Error)]
#[error("invalid multihash")]
pub struct InvalidMultihash;

/// Returned when a block lookup fails.
#[derive(Debug, Error)]
#[error("block not found {0}")]
pub struct BlockNotFound(pub Cid);
