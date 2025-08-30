//! Module with tools for visualizations of checksums, hashes and
//! other cryptographic objects
//!
//! This module provides some basic tools for visualizing checksums
//! and other cryptographic objects.
//!
//! This module uses the std library, but the checksum algorithms do
//! not require the std library.
#![warn(missing_docs)]
#![warn(unsafe_code)]

pub mod shift_register_diagram;
