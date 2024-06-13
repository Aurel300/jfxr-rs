//! Rust port of the [`jfxr`](https://github.com/ttencate/jfxr) sound effect
//! generation tool/library.
//!
//! # Saving and loading `.jfxr` files
//!
//! This crate is compatible with the `jfxr` format produced by the `jfxr`
//! tool. When the `json` feature is enabled, `.jfxr` files can be parsed and
//! encoded into instances of [`Sound`]:
//!
//! ```rust
//! let sound_data = std::fs::read_to_string("example.jfxr").unwrap();
//! let mut sound = jfxr::read_jfxr(sound_data).unwrap();
//! sound.frequency.0 = 200.0;
//! let new_sound_data = jfxr::write_jfxr(sound);
//! std::fs::write("new_example.jfxr", new_sound_data).unwrap();
//! ```
//!
//! # Generating samples
//!
//! Sound samples in the form of a [`Vec<f64>`] can be generated from a
//! [`Sound`]:
//!
//! ```rust
//! let samples = jfxr::generate(sound);
//! ```
//!
//! By default, [`generate`] generates single-channel samples at a 44100 Hz
//! sample rate, and the entire sound is generated in a single synchronous
//! call. For more control, create an instance of [`Synth`] with a reference
//! to a [`Sound`]. Output settings can be adjusted on the [`Synth`] instance,
//! and the generation can be split across multiple calls to
//! [`Synth::generate_block`].

#[cfg(feature = "json")]
pub mod jfxr;
pub mod oscillator;
pub mod parameter;
pub mod sound;
pub mod synth;

#[cfg(feature = "json")]
pub use jfxr::{read_jfxr, write_jfxr};
pub use sound::Sound;
pub use synth::Synth;

/// Generates the given [`Sound`] sound into samples. The output vector
/// contains single-channel samples at a 44100 Hz sample rate, and the entire
/// sound is generated in a single synchronous call. For more control, create
/// an instance of [`Synth`] with a reference to a [`Sound`]. Output settings
/// can be adjusted on the [`Synth`] instance, and the generation can be split
/// across multiple calls to [`Synth::generate_block`].
pub fn generate(sound: &Sound) -> Vec<f64> {
    Synth::new(sound).generate()
}
