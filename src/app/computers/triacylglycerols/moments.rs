use crate::utils::HashedDataFrame;
use egui::util::cache::{ComputerMut, FrameCache};
use polars::prelude::*;
use std::iter::once;
use tracing::instrument;

/// Moments computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Moments computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<DataFrame> {
        let mut lazy_frame = key.frame.data_frame.clone().lazy();
        println!("Moments 0: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = lazy_frame.select([all().exclude_cols(["Composition", "Species"]).as_expr()]);
        // to_ndarray
        // lazy_frame = lazy_frame.select([nth(1)
        //     .as_expr()
        //     .struct_()
        //     .field_by_name("Mean")
        //     .fill_null(0)
        //     .apply(
        //         |column| {
        //             use ndarray::array;
        //             use scirs2::stats::{kurtosis, mean, median, skew, std, var};
        //             let x = array![1.0, 2.0, 3.0, 4.0, 5.0];
        //             let y = array![5.0, 4.0, 3.0, 2.0, 1.0];
        //             let mean = mean(&x.view()).unwrap(); // 3.0
        //             // let median = median(&x.view()).unwrap(); // 3.0
        //             // let std = std(&x.view(), 1, None).unwrap(); // Sample std dev
        //             // let skewness = skew(&x.view(), false, None).unwrap();
        //             // let kurt = kurtosis(&x.view(), true, false, None).unwrap();
        //             // println!("mean: {mean}; median: {median}; std: {std}; skewness: {skewness}; kurt: {kurt}");
        //             Ok(column)
        //         },
        //         |_, field| Ok(field.clone()),
        //     )]);
        println!("Moments x: {}", lazy_frame.clone().collect().unwrap());
        let schema = lazy_frame.collect_schema()?;
        let skewness = lazy_frame.clone().select(
            once(lit("Skewness").alias("Moment"))
                .chain(schema.iter_names_cloned().map(|name| {
                    col(name.clone())
                        .struct_()
                        .field_by_name("Mean")
                        .fill_null(0)
                        .skew(key.bias)
                        .alias(name)
                }))
                .collect::<Vec<_>>(),
        );
        let kurtosis = lazy_frame.select(
            once(lit("Kurtosis").alias("Moment"))
                .chain(schema.iter_names_cloned().map(|name| {
                    col(name.clone())
                        .struct_()
                        .field_by_name("Mean")
                        .fill_null(0)
                        .kurtosis(true, key.bias)
                        .alias(name)
                }))
                .collect::<Vec<_>>(),
        );
        let lazy_frame = concat([skewness, kurtosis], UnionArgs::default())?;
        println!("Moments 2: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame.collect()
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Moments key
#[derive(Clone, Copy, Debug, Hash)]
pub(crate) struct Key<'a> {
    pub(crate) frame: &'a HashedDataFrame,
    pub(crate) bias: bool,
}

/// Moments value
type Value = DataFrame;

// fn kurtosis() -> Expr {
// }
