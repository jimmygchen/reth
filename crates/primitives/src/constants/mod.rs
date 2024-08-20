//! Ethereum protocol-related constants

pub use reth_primitives_traits::constants::*;

/// [EIP-4844](https://eips.ethereum.org/EIPS/eip-4844#parameters) constants.
pub mod eip4844;
pub use eip4844::{MAX_BLOBS_PER_BLOCK, MAX_DATA_GAS_PER_BLOCK, TARGET_DATA_GAS_PER_BLOCK, TARGET_BLOBS_PER_BLOCK};