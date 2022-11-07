//! Module with tools for analysis of checksums, hashes and other cryptographic objects
//!
//! This module provides some basic tools for analyzing strength of
//! checksums and other cryptographic objects.
//! It is not a replacement for a full statistical analysis crate.  It
//! also has not been tested as thoroughly as other libraries for
//! statistics.  It can be used to show some of the statistical tools
//! available for looking at checksums, but should not be used as the
//! final or only basis for selecting an algorithm.
//!
//! This module uses the std library, but the checksum algorithms do
//! not require the std library.
//!
//! This module is not IEEE 758 compliant
//!
//! See [`impl Ord for Sample<f32>`](sample::Sample#impl-Ord-for-Sample<f32>)
#![warn(missing_docs)]
#![warn(unsafe_code)]

pub mod distribution;
pub mod experiment;
pub mod histogram;
pub mod ks;
#[cfg(feature = "external-rand")]
pub mod rand_distribution;
pub mod sample;
