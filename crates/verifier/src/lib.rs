//! This crate provides verifiers for SP1 Groth16 and Plonk BN254 proofs in a no-std environment.
//! It is patched for efficient verification within the SP1 zkVM context.

#![cfg_attr(not(any(feature = "std", test)), no_std)]
extern crate alloc;

use lazy_static::lazy_static;
#[cfg(feature = "recursion")]
use slop_algebra::PrimeField;
#[cfg(feature = "recursion")]
use sp1_hypercube::koalabears_to_bn254;

lazy_static! {
    /// The PLONK verifying key for this SP1 version.
    pub static ref PLONK_VK_BYTES: &'static [u8] = include_bytes!("../vk-artifacts/plonk_vk.bin");
}

lazy_static! {
    /// The Groth16 verifying key for this SP1 version.
    pub static ref GROTH16_VK_BYTES: &'static [u8] = include_bytes!("../vk-artifacts/groth16_vk.bin");
}

#[cfg(any(test, not(feature = "recursion")))]
const VK_ROOT_PRECOMPUTED: [u8; 32] = [
    0x00, 0x2f, 0x85, 0x0e, 0xe9, 0x98, 0x97, 0x4d, 0x6c, 0xc0, 0x0e, 0x50, 0xcd, 0x08, 0x14, 0xb0,
    0x98, 0xc0, 0x5b, 0xfa, 0xde, 0x46, 0x6d, 0x28, 0x57, 0x32, 0x40, 0xd0, 0x57, 0xf2, 0x53, 0x52,
];

#[cfg(feature = "recursion")]
fn vk_root() -> [u8; 32] {
    let vks = recursion_vks::VerifierRecursionVks::default();
    let be_bytes = koalabears_to_bn254(&vks.root()).as_canonical_biguint().to_bytes_be();
    let mut root = [0u8; 32];
    root[32 - be_bytes.len()..].copy_from_slice(&be_bytes);
    root
}

#[cfg(not(feature = "recursion"))]
fn vk_root() -> [u8; 32] {
    VK_ROOT_PRECOMPUTED
}

lazy_static! {
    pub static ref VK_ROOT_BYTES: [u8; 32] = vk_root();
}

#[cfg(all(test, feature = "recursion"))]
#[test]
fn precomputed_vk_root_matches_recursion_derivation() {
    assert_eq!(vk_root(), VK_ROOT_PRECOMPUTED);
}

#[cfg(feature = "recursion")]
mod recursion_vks;
#[cfg(feature = "recursion")]
pub use recursion_vks::VerifierRecursionVks;

#[cfg(feature = "recursion")]
pub mod compressed;

mod constants;
pub mod converter;
mod error;
#[cfg(feature = "recursion")]
mod proof;

mod utils;
pub use utils::*;

pub use groth16::{error::Groth16Error, Groth16Verifier};
#[cfg(feature = "recursion")]
pub use proof::*;
mod groth16;

#[cfg(feature = "ark")]
pub use groth16::ark_converter::*;

pub use plonk::{error::PlonkError, PlonkVerifier};
mod plonk;

#[cfg(test)]
mod tests;
