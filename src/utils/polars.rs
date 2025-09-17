use polars::prelude::*;
use std::fmt::{Display, from_fn};

pub fn hash_data_frame(data_frame: &mut DataFrame) -> PolarsResult<u64> {
    Ok(data_frame.hash_rows(None)?.xor_reduce().unwrap_or_default())
}

/// Extension methods for [`AnyValue`]
pub trait AnyValueExt {
    fn display(&self) -> String;
}

impl AnyValueExt for AnyValue<'_> {
    fn display(&self) -> String {
        from_fn(|f| match self {
            AnyValue::Null => f.write_str("-"),
            _ => Display::fmt(&self, f),
        })
        .to_string()
    }
}
