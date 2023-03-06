#![forbid(unsafe_code)]
#![no_std]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(rustdoc::broken_intra_doc_links)]
//! A generic trait for converting [`tuple`] to type [`array`].
//!
//! Usage:
//!
//! ```
//! # use try_tup_to_arr::TryTupToArr;
//! assert_eq!(TryTupToArr::<i32>::try_into_arr((45u8, 2023u16, -60i8,)),
//! Ok([45i32, 2023i32, -60i32]));
//! assert_eq!(TryTupToArr::<i16>::try_into_arr(("45", 2023u16, true,)),
//! Ok([45i16, 2023i16, 1i16]));
//! assert_eq!(TryTupToArr::try_into_arr((45u8, 2023u16, -53i8,))
//! .unwrap().iter().sum::<i32>(), 2015i32);
//!assert!(TryTupToArr::<i16>::try_into_arr(("6032023", 2023u16, true,)).is_err());
//! ```
#[doc = include_str!("../README.md")]
use core::fmt;
use core::fmt::Display;
use try_from_int_str::{IntStrError, TryFromIntStr, TryFromIntStrErr};
extern crate try_tup_to_arr_macro;
use try_tup_to_arr_macro::try_tup_to_arr_impl;

/// A structure to store the error and its position.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryTupToArrErr {
    pub(crate) source: TryFromIntStrErr,
    posice: usize,
}

impl Display for TryTupToArrErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.source.multi_err() {
            IntStrError::ErrorStr(parse_int_error) => {
                write!(f, "{parse_int_error}")
            }
            IntStrError::ErrorInt(try_from_int_error) => {
                write!(f, "{try_from_int_error}")
            }
        }
    }
}

impl From<TryFromIntStrErr> for TryTupToArrErr {
    fn from(err: TryFromIntStrErr) -> Self {
        Self {
            source: err,
            posice: 0,
        }
    }
}

/// A trait to convert a [`tuple`] to an [`array`] of integers, a possible conversion error.
pub trait TryTupToArr<U> {
    ///Output type [`array`] of integers.
    type A;

    /// Returns an [`array`] of integers or a conversion error.
    fn try_into_arr(self) -> Result<Self::A, TryTupToArrErr>;
}

try_tup_to_arr_impl!();
