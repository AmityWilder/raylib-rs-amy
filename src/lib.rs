/// Safe-ish wrappers for ffi functions
///
/// This is the bare minimum to prevent UB
///
/// The [`safe`] module should only ever call into Raylib-C
/// through this module, never [`low_level::sys`] itself.
pub mod low_level;

/// High-level code for covering edge cases and best practice
///
/// Adds ergonomics and a safety net against carelessness
pub mod safe;

pub mod prelude {
    pub use crate::safe::{
        into_cstr::*,
        *,
    };
}

mod tests;
