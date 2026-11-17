#[cfg(feature = "std")]
use std::path::PathBuf;

use crate::prelude::*;

/// Errors from ffcine and its backends.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[cfg(feature = "std")]
    #[error("{path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[cfg(feature = "std")]
    #[error("{path}:{line}: invalid .cube LUT: {msg}")]
    CubeParse {
        path: PathBuf,
        line: usize,
        msg: String,
    },
    #[error("invalid {what}: {value:?}")]
    Invalid { what: &'static str, value: String },
    /// A param track that doesn't parse or evaluate.
    #[error("{path}: {msg}")]
    Track { path: String, msg: String },
    /// An expression that doesn't compile: the column (1-based, characters) and a caret.
    #[error("{path}: col {col}: {msg}\n  {src}\n  {caret}^", caret = " ".repeat(col.saturating_sub(1)))]
    Expr {
        path: String,
        col: usize,
        msg: String,
        src: String,
    },
    #[cfg(feature = "std")]
    #[error("{path}: {msg}")]
    Import { path: PathBuf, msg: String },
    #[error("invalid OTIO JSON: {0}")]
    Json(#[from] serde_json::Error),
    /// The selected backend cannot run this (feature disabled, or not built yet).
    #[error("{0} is not implemented yet")]
    Unimplemented(&'static str),
    /// GPU pipeline failure (device setup, libplacebo, hw frames, codecs).
    #[error("{0}")]
    Pipeline(String),
    /// A GPU API call failed (CUDA, NVIDIA optical flow).
    #[error("{api}: {msg}")]
    Gpu { api: &'static str, msg: String },
    #[cfg(feature = "ffmpeg")]
    #[error("ffmpeg: {0}")]
    Ffmpeg(#[from] ffmpeg_next::Error),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
