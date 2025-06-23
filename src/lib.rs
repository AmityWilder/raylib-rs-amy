#![warn(clippy::missing_safety_doc, clippy::undocumented_unsafe_blocks, clippy::missing_panics_doc)]

/// Low-level wrappers for ffi functions
///
/// Used for providing a Rust-style interface with C functions, ensuring buffers
/// are the correct size, integers are unsigned where relevant, and static pointers
/// cannot be mutated while being referenced--but does not ensure initialization or
/// drop conditions.
///
/// The [`safe`] module should only ever call into Raylib-C through this module,
/// never [`low::sys`] itself (however [`low::sys`] types may be used by [`safe`]).
///
/// ---
///
/// # Code to review every Raylib update
///
/// Some Raylib C code that does not involve state mutation or platform-specific interaction
/// has been directly translated into Rust so that it can be made `const` and inlined. This
/// code must be reviewed each time the Raylib submodule is updated to ensure parity.
///
/// **Please review the following code every time the Raylib submodule is updated:**
///
/// - [`low::textures::get_pixel_data_size()`]
/// - [`low::textures::is_image_valid()`]
/// - [`low::audio::is_music_valid()`]
pub mod low;

/// High-level code for covering edge cases and best practice
///
/// Adds ergonomics, ensures resources are initialized before use, and frees weak resources
/// when strong handles drop.
pub mod safe;

pub mod prelude {
    pub use crate::safe::{
        into_cstr::*,
        *,
    };
}

mod tests;
