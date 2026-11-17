//! ffcine core types shared by every other ffcine crate: the [`Error`] type, param tracks and
//! expressions ([`anim`]), colour spaces
//! ([`ColorSpace`]), 3D LUTs ([`Lut`]), probed media info ([`MediaInfo`]) and 2D geometry
//! ([`geom`]), camera input profiles and their LUTs ([`camera`]), and libav-free probes for camera
//! files: ISO BMFF ([`bmff`]), WAV ([`wav`]) and picture profile detection ([`profile`]). No GPU or
//! ffmpeg dependency (feature `ffmpeg` only adds `Error::Ffmpeg`).
//!
//! `no_std` + `alloc` without the default `std` feature: [`anim`], [`color`], [`geom`],
//! [`profile`], the [`bmff`] types and [`Error`] remain. `std` adds the file readers
//! (`bmff::read`, `wav`), `.cube` LUTs (`lut`, whose `Lut` keeps the `PathBuf` it was loaded
//! from), `MediaInfo` (a `PathBuf` too), `camera` (built on both, plus `$FFCINE_LUT_DIR` and XDG
//! lookups) and the `Io`, `CubeParse` and `Import` error variants.

#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod anim;
pub mod bmff;
#[cfg(feature = "std")]
pub mod camera;
pub mod color;
mod error;
pub mod geom;
#[cfg(feature = "std")]
pub mod lut;
mod prelude;
pub mod probe;
pub mod profile;
#[cfg(feature = "std")]
pub mod wav;

pub use color::ColorSpace;
pub use error::{Error, Result};
#[cfg(feature = "std")]
pub use lut::{Lut, LutType};
#[cfg(feature = "std")]
pub use probe::MediaInfo;
pub use probe::Rational;
pub use profile::{Confidence, PictureProfile, ProfileGuess};
