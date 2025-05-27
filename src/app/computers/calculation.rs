use crate::app::panes::calculation::settings::Settings;
use egui::util::cache::{ComputerMut, FrameCache};
use polars::prelude::*;
use polars_ext::{prelude::ExprExt, series::column};
use std::{
    hash::{Hash, Hasher},
    iter::zip,
};

/// Calculation computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Calculation computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    fn try_compute(&mut self, key: Key) -> PolarsResult<DataFrame> {
        let mut lazy_frame = key.data_frame.clone().lazy();
        // let other = MATURE_MILK.data.clone().lazy().select([
        //     col("FattyAcid").hash(),
        //     col("FattyAcid"),
        //     col("StereospecificNumber123").alias("Target123"),
        //     col("StereospecificNumber2").alias("Target2"),
        // ]);
        // if !key.data_frame.is_empty() {
        //     lazy_frame = lazy_frame
        //         .select([
        //             col("FattyAcid").hash(),
        //             col("FattyAcid"),
        //             col("StereospecificNumber123").alias("Source123"),
        //             col("StereospecificNumber2").alias("Source2"),
        //         ])
        //         .join(
        //             other,
        //             &[col("Hash"), col("FattyAcid")],
        //             &[col("Hash"), col("FattyAcid")],
        //             JoinArgs::new(JoinType::Left),
        //         )
        //         .drop(["Hash"])
        //         .with_column(col("FattyAcid"))
        //         .with_column(if key.settings.relative {
        //             lit(100) * col("Source2") / col("Source123") / lit(3)
        //         } else {
        //             col("Source2")
        //         });
        //     let output_type = GetOutput::from_type(DataType::Struct(vec![
        //         Field::new(
        //             "Data".into(),
        //             DataType::Struct(vec![
        //                 Field::new("A".into(), DataType::Float64),
        //                 Field::new("B".into(), DataType::Float64),
        //                 Field::new("C".into(), DataType::Float64),
        //                 Field::new("D".into(), DataType::Float64),
        //                 Field::new("E".into(), DataType::Float64),
        //                 Field::new("F".into(), DataType::Float64),
        //             ]),
        //         ),
        //         Field::new(
        //             "Meta".into(),
        //             DataType::Struct(vec![
        //                 Field::new("Min".into(), DataType::Float64),
        //                 Field::new("Max".into(), DataType::Float64),
        //                 Field::new("Sum".into(), DataType::Float64),
        //             ]),
        //         ),
        //     ]));
        //     println!("calculate0: {}", lazy_frame.clone().collect().unwrap());
        //     lazy_frame =
        //         lazy_frame.with_columns([as_struct(vec![col("Source123"), col("Target123")])
        //             .apply(column(abcdef(&key.settings)), output_type.clone())
        //             .alias("StereospecificNumber123")]);
        //     println!("calculate1: {}", lazy_frame.clone().collect().unwrap());
        //     lazy_frame = lazy_frame.with_columns([as_struct(vec![col("Source2"), col("Target2")])
        //         .apply(column(abcdef(&key.settings)), output_type)
        //         .alias("StereospecificNumber2")]);
        //     lazy_frame = lazy_frame.with_columns([(col("StereospecificNumber123")
        //         .struct_()
        //         .field_by_name("Data")
        //         .struct_()
        //         .field_by_name("E")
        //         .fill_null(0)
        //         + col("StereospecificNumber2")
        //             .struct_()
        //             .field_by_name("Data")
        //             .struct_()
        //             .field_by_name("E")
        //             .fill_null(0))
        //     .alias("F")]);
        //     // lazy_frame = lazy_frame.with_column(
        //     //     sum_horizontal(
        //     //         [
        //     //             col("StereospecificNumber123")
        //     //                 .struct_()
        //     //                 .field_by_name("Data")
        //     //                 .struct_()
        //     //                 .field_by_name("E"),
        //     //             col("StereospecificNumber2")
        //     //                 .struct_()
        //     //                 .field_by_name("Data")
        //     //                 .struct_()
        //     //                 .field_by_name("E"),
        //     //         ],
        //     //         true,
        //     //     )?
        //     //     .alias("F"),
        //     // );
        //     println!("calculate !!!!: {}", lazy_frame.clone().collect().unwrap());
        //     lazy_frame = lazy_frame.select([
        //         col("StereospecificNumber123"),
        //         col("StereospecificNumber2"),
        //         col("F"),
        //     ]);
        // }
        lazy_frame.collect()
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Calculation key
#[derive(Clone, Copy, Debug)]
pub(crate) struct Key<'a> {
    pub(crate) data_frame: &'a DataFrame,
    pub(crate) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for series in self.data_frame.iter() {
            for value in series.iter() {
                value.hash(state);
            }
        }
        self.settings.hash(state);
    }
}

/// Calculation value
type Value = DataFrame;
