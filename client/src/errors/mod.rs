// Error module for Miden Lending Protocol
// This module contains error constants extracted from MASM code

#[cfg(any(feature = "testing", test))]
pub mod lending_errors;

#[cfg(any(feature = "testing", test))]
pub use lending_errors::*;
