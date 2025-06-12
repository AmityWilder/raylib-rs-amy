/// Safe-ish wrappers for ffi functions
///
/// This is the bare minimum to prevent UB
pub mod low_level;

/// High-level code for covering edge cases and best practice
///
/// Adds ergonomics and a safety net against carelessness
pub mod safe;
