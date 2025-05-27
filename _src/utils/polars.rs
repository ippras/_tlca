use polars::prelude::*;
use std::fmt::{Display, from_fn};

// TODO: Следить когда добавят возможность складывать массивы поэлементно
// format_array(
//     expr
//         .arr()
//         .eval(element().sum(), false),
//     key,
// )?,
pub fn sum_arr(expr: Expr) -> PolarsResult<Expr> {
    concat_arr(vec![
        expr.arr()
            .to_struct(None)
            .struct_()
            .field_by_name("*")
            .sum(),
    ])
}

pub fn eval_arr(expr: Expr, f: impl Fn(Expr) -> Expr) -> PolarsResult<Expr> {
    concat_arr(vec![f(expr
        .arr()
        .to_struct(None)
        .struct_()
        .field_by_name("*"))])
}

// /// Extension methods for [`AnyValue`]
// pub trait AnyValueExt {
//     fn display(&self) -> String;
// }

// impl AnyValueExt for AnyValue<'_> {
//     fn display(&self) -> String {
//         from_fn(|f| match self {
//             AnyValue::Null => f.write_str("-"),
//             _ => Display::fmt(&self, f),
//         })
//         .to_string()
//     }
// }
