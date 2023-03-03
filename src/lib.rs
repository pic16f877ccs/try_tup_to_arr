use core::fmt;
use core::fmt::Display;
use try_from_int_str::{TryFromIntStr, TryFromIntStrErr, IntStrError};
extern crate try_tup_to_arr_macro;
use try_tup_to_arr_macro::{ try_tup_to_arr_impl };

#[doc = "Conversion errors."]
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
        Self { source: err, posice: 0 }
    }
}

pub trait TryTupToArr<U> {
    type A;
    fn try_into_arr(self) -> Result<Self::A, TryTupToArrErr>;
}

try_tup_to_arr_impl!();
