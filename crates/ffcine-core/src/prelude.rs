//! What `std`'s prelude would bring in, from `alloc`; and, without `std`, the float maths
//! (`sqrt`, `powf`, ...) from `libm` through `num_traits::Float`. With `std` the inherent methods
//! win, so results don't change.

#[allow(unused_imports)]
pub(crate) use alloc::{
    borrow::ToOwned,
    boxed::Box,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub(crate) use num_traits::Float as _;
