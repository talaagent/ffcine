//! What `std`'s prelude would bring in, from `alloc`.

#[allow(unused_imports)]
pub(crate) use alloc::{
    borrow::ToOwned,
    boxed::Box,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
