#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate bench_utils;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate alloc;

#[cfg(not(feature = "std"))]
pub(crate) use alloc::{borrow::ToOwned, boxed::Box, vec::Vec};

#[cfg(feature = "std")]
pub(crate) use std::{borrow::ToOwned, boxed::Box, vec::Vec};

pub mod commitment;
pub mod crh;
pub mod merkle_tree;
pub mod nizk;
pub mod prf;
pub mod signature;

pub use self::{
    commitment::CommitmentScheme,
    crh::FixedLengthCRH,
    merkle_tree::{MerkleHashTree, MerkleTreePath},
    nizk::NIZK,
    prf::PRF,
    signature::SignatureScheme,
};

#[cfg(feature = "r1cs")]
pub use self::{
    commitment::CommitmentGadget, crh::FixedLengthCRHGadget,
    merkle_tree::constraints::MerkleTreePathGadget, nizk::NIZKVerifierGadget, prf::PRFGadget,
    signature::SigRandomizePkGadget,
};

#[cfg(feature = "std")]
pub type Error = Box<dyn std::error::Error>;

#[cfg(not(feature = "std"))]
pub type Error = Box<dyn algebra::Error>;

#[derive(Debug)]
pub enum CryptoError {
    IncorrectInputLength(usize),
    NotPrimeOrder,
}

impl core::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            CryptoError::IncorrectInputLength(len) => format!("input length is wrong: {}", len),
            CryptoError::NotPrimeOrder => "element is not prime order".to_owned(),
        };
        write!(f, "{}", msg)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CryptoError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[cfg(not(feature = "std"))]
impl algebra::Error for CryptoError {}
