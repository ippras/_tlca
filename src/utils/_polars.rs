use polars::prelude::*;
use std::fmt::{Display, from_fn};

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
