//! An interface for defining spatial audio rendering modes and
//! implementing mode-agnostic renderers for the xpans Ecosystem
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
mod std;

pub mod input;
pub mod interpret;
pub mod output;
pub mod process;

pub mod prelude;
