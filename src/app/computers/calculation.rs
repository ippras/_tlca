use crate::{
    app::panes::calculation::settings::{Kind, Settings},
    utils::Hashed,
};
use egui::util::cache::{ComputerMut, FrameCache};
use metadata::MetaDataFrame;
use polars::prelude::*;
use polars_ext::{prelude::ExprExt, series::column};
use std::hash::{Hash, Hasher};
use tracing::instrument;

/// Calculation computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Calculation computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<DataFrame> {
        if key.frames.is_empty() {
            return Ok(DataFrame::empty());
        }
        let compute = |frame: &MetaDataFrame| -> PolarsResult<LazyFrame> {
            Ok(frame.data.clone().lazy().select([
                col("Triacylglycerol"),
                col("Value").alias(frame.meta.format(".").to_string()),
            ]))
        };
        println!("key.frames: {:?}", key.frames);
        let mut lazy_frame = compute(&key.frames[0])?;
        for frame in &key.frames[1..] {
            lazy_frame = lazy_frame.join(
                compute(frame)?,
                [col("Triacylglycerol")],
                [col("Triacylglycerol")],
                JoinArgs {
                    coalesce: JoinCoalesce::CoalesceColumns,
                    maintain_order: MaintainOrderJoin::LeftRight,
                    ..JoinArgs::new(JoinType::Full)
                },
            );
        }
        match key.settings.kind {
            Kind::Value => {}
            Kind::Difference => {
                lazy_frame = lazy_frame.select([
                    col("Triacylglycerol"),
                    (dtype_col(&DataType::Float64) - nth(1)).abs(),
                    // (dtype_col(&DataType::Float64) - nth(1)).abs() / nth(1),
                ]);
            }
            Kind::Jaccard => {
                // let a = dtype_col(&DataType::Float64).drop_nulls().len();
                // let b = nth(1).drop_nulls().len();
                // let c = dtype_col(&DataType::Float64)
                //     .is_not_null()
                //     .and(nth(1).is_not_null())
                //     .sum();
                // lazy_frame = lazy_frame.select([
                //     col("Triacylglycerol"),
                //     c.clone() / (a + b - c).cast(DataType::Float64),
                // ]);
                // lazy_frame = lazy_frame.select([col("Triacylglycerol"), c]);
            }
        }
        println!("lazy_frame: {:?}", lazy_frame.clone().collect()?);
        lazy_frame.collect()
        // let mut lazy_frame = match settings.index {
        //     Some(index) => {
        //         let frame = &key.frames[index];
        //         let mut lazy_frame = frame.data.clone().lazy();
        //         lazy_frame = compute(lazy_frame, settings)?;
        //         lazy_frame
        //     }
        //     None => {
        //         let compute = |frame: &MetaDataFrame| -> PolarsResult<LazyFrame> {
        //             Ok(compute(frame.data.clone().lazy(), settings)?.select([
        //                 col("Keys").hash(),
        //                 col("Keys"),
        //                 col("Values").alias(frame.meta.title()),
        //             ]))
        //         };
        //         let mut lazy_frame = compute(&key.frames[0])?;
        //         for frame in &key.frames[1..] {
        //             lazy_frame = lazy_frame.join(
        //                 compute(frame)?,
        //                 [col("Hash"), col("Keys")],
        //                 [col("Hash"), col("Keys")],
        //                 JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
        //             );
        //         }
        //         lazy_frame = lazy_frame.drop([col("Hash")]);
        //         lazy_frame = meta(lazy_frame, settings)?;
        //         lazy_frame
        //     }
        // };

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
    }
}

// /// Jaccard
// fn jaccard(exp1: Expr, exp2: Expr) -> PolarsResult<f64> {
//     let (s1, s2) = (exp1.join().unique(), exp1.unique());
//     let s3 = s1
//         .to_frame()
//         .join(s2.to_frame(), how = "inner", on = "")
//         .to_series();
//     return s3.len() / (s1.len() + s2.len() - s3.len());
// }

// def jaccard_similarity(s1: pl.Series, s2: pl.Series):
//     s1, s2 = s1.unique(), s2.unique()
//     s3 = s1.to_frame().join(s2.to_frame(), how="inner", on="").to_series()
//     return s3.len() / (s1.len() + s2.len() - s3.len())

// def jaccard_distance(s1: pl.Series, s2: pl.Series):
//     return 1 - jaccard_similarity(s1, s2)

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Calculation key
#[derive(Clone, Copy, Debug)]
pub(crate) struct Key<'a> {
    pub(crate) frames: &'a [Hashed<MetaDataFrame>],
    pub(crate) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.frames.hash(state);
        self.settings.kind.hash(state);
    }
}

/// Calculation value
type Value = DataFrame;
