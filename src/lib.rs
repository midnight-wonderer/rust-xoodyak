#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod error;
mod sparklep;
mod xoospark;

pub use crate::error::Error as XoosparkError;
pub use crate::sparklep::SparkleP;
pub use crate::xoospark::{
    Tag as XoosparkTag, XoosparkAny, XoosparkCommon, XoosparkHash, XoosparkKeyed,
    AUTH_TAG_BYTES as XOOSPARK_AUTH_TAG_BYTES,
};

#[cfg(test)]
mod test;
