use crate::{
    app::states::{
        Filter, Sort,
        fatty_acids::{Display, Settings, StereospecificNumbers},
    },
    utils::{HashedDataFrame, HashedMetaDataFrame},
};
use egui::{
    emath::OrderedFloat,
    util::cache::{ComputerMut, FrameCache},
};
use lipid::prelude::*;
use polars::prelude::*;
use tracing::instrument;

/// Fatty acids computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Fatty acids computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        if key.frames.is_empty() {
            return Ok(HashedDataFrame::EMPTY);
        }
        let lazy_frame = compute(key)?;
        let data_frame = lazy_frame.collect()?;
        HashedDataFrame::new(data_frame)
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Fatty acids key
#[derive(Clone, Copy, Debug, Hash)]
pub(crate) struct Key<'a> {
    pub(crate) frames: &'a [HashedMetaDataFrame],
    pub(crate) display: Display,
    pub(crate) stereospecific_numbers: StereospecificNumbers,
    pub(crate) filter: Filter,
    pub(crate) sort: Sort,
    pub(crate) threshold: OrderedFloat<f64>,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frames: &'a [HashedMetaDataFrame], settings: &Settings) -> Self {
        Self {
            frames,
            display: settings.parameters.display,
            stereospecific_numbers: settings.parameters.stereospecific_numbers,
            filter: settings.parameters.filter,
            sort: settings.parameters.sort,
            threshold: settings.parameters.threshold.into(),
        }
    }
}

/// Fatty acids value
type Value = HashedDataFrame;

fn compute(key: Key) -> PolarsResult<LazyFrame> {
    println!("Fatty acids frames: {:?}", key.frames);
    let mut lazy_frame = join(key)?;
    println!(
        "Fatty acids join: {}",
        lazy_frame.clone().collect().unwrap()
    );
    lazy_frame = values(lazy_frame, key)?;
    println!(
        "Fatty acids values: {}",
        lazy_frame.clone().collect().unwrap()
    );
    lazy_frame = filter(lazy_frame, key)?;
    println!(
        "Fatty acids filter: {}",
        lazy_frame.clone().collect().unwrap()
    );
    lazy_frame = sort(lazy_frame, key);
    // println!(
    //     "Fatty acids sort: {}",
    //     lazy_frame.clone().collect().unwrap()
    // );
    Ok(lazy_frame)
}

/// Join
fn join(key: Key) -> PolarsResult<LazyFrame> {
    let compute = |frame: &HashedMetaDataFrame| -> PolarsResult<LazyFrame> {
        Ok(frame.data.data_frame.clone().lazy().select([
            col(LABEL),
            col(FATTY_ACID),
            as_struct(vec![
                col(STEREOSPECIFIC_NUMBERS123),
                col(STEREOSPECIFIC_NUMBERS13),
                col(STEREOSPECIFIC_NUMBERS2),
            ])
            .alias(frame.meta.format(".").to_string()),
        ]))
    };
    let mut lazy_frame = compute(&key.frames[0])?;
    for frame in &key.frames[1..] {
        lazy_frame = lazy_frame.join(
            compute(frame)?,
            [col(LABEL), col(FATTY_ACID)],
            [col(LABEL), col(FATTY_ACID)],
            JoinArgs {
                coalesce: JoinCoalesce::CoalesceColumns,
                maintain_order: MaintainOrderJoin::LeftRight,
                ..JoinArgs::new(JoinType::Full)
            },
        );
    }
    Ok(lazy_frame)
}

/// Values
fn values(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let schema = lazy_frame.collect_schema()?;
    match key.display {
        Display::StereospecificNumbers => {
            let r#struct = |name: &PlSmallStr| match key.stereospecific_numbers {
                StereospecificNumbers::Sn123 => col(name.clone())
                    .struct_()
                    .field_by_name(STEREOSPECIFIC_NUMBERS123),
                StereospecificNumbers::Sn1 => col(name.clone())
                    .struct_()
                    .field_by_name(STEREOSPECIFIC_NUMBERS13),
                StereospecificNumbers::Sn2 => col(name.clone())
                    .struct_()
                    .field_by_name(STEREOSPECIFIC_NUMBERS2),
                StereospecificNumbers::Sn3 => col(name.clone())
                    .struct_()
                    .field_by_name(STEREOSPECIFIC_NUMBERS13),
            };
            let exprs = schema
                .iter_names()
                .filter(|&name| name != LABEL && name != FATTY_ACID)
                .map(|name| {
                    let mean = r#struct(name).struct_().field_by_name("Mean");
                    let standard_deviation =
                        r#struct(name).struct_().field_by_name("StandardDeviation");
                    let array = r#struct(name).struct_().field_by_name("Array");
                    ternary_expr(
                        mean.clone().neq(0),
                        as_struct(vec![
                            mean.alias("Mean"),
                            standard_deviation.alias("StandardDeviation"),
                            array.alias("Array"),
                        ]),
                        lit(NULL),
                    )
                    .alias(name.clone())
                })
                .collect::<Vec<_>>();
            lazy_frame = lazy_frame.with_columns(exprs);
        }
        Display::Indices => {
            // let mut lazy_frames = Vec::with_capacity(INDICES_LIST.len());
            // for index in [] {
            //     let exprs = schema
            //         .iter_names()
            //         .filter(|&name| name != LABEL && name != FATTY_ACID)
            //         .map(|name| {
            //             let expr = col(name.clone())
            //                 .struct_()
            //                 .field_by_name(match key.stereospecific_numbers {
            //                     StereospecificNumbers::Sn123 => STEREOSPECIFIC_NUMBERS123,
            //                     StereospecificNumbers::Sn1 => STEREOSPECIFIC_NUMBERS13,
            //                     StereospecificNumbers::Sn2 => STEREOSPECIFIC_NUMBERS2,
            //                     StereospecificNumbers::Sn3 => STEREOSPECIFIC_NUMBERS13,
            //                 })
            //                 .struct_()
            //                 .field_by_name("Mean");
            //             let mean = col(FATTY_ACID).fatty_acid().unsaturated(expr, None);
            //             let standard_deviation = lit(0.0).alias("StandardDeviation");
            //             let array = concat_arr(vec![lit(0.0)]).unwrap().alias("Array");
            //             as_struct(vec![
            //                 mean.alias("Mean"),
            //                 standard_deviation.alias("StandardDeviation"),
            //                 array.alias("Array"),
            //             ])
            //             .alias(name.clone())
            //         })
            //         .collect::<Vec<_>>();
            //     lazy_frames.push(lazy_frame.clone().select(exprs));
            // }
            // lazy_frame = concat(lazy_frames, Default::default())?;
            // lazy_frame = lazy_frame.select([
            //     lit(Series::new(
            //         PlSmallStr::from_static(LABEL),
            //         [
            //             "Monounsaturated",
            //             "Polyunsaturated",
            //             "Saturated",
            //             "Trans",
            //             "Unsaturated",
            //         ],
            //     )),
            //     lit(Series::from_any_values(
            //         PlSmallStr::from_static(FATTY_ACID),
            //         &vec![fatty_acid!(C0 {})?; INDICES_LIST.len()],
            //         true,
            //     )?),
            //     all().as_expr(),
            // ]);
            println!(
                "azy_frame.select([index]).collect().unwrap(): {:?}",
                lazy_frame.clone().collect().unwrap()
            );

            // let exprs = [
            //     lit(Series::new(
            //         PlSmallStr::from_static(LABEL),
            //         ["Monounsaturated"],
            //     )),
            //     lit(Series::from_any_values(
            //         FATTY_ACID.into(),
            //         &[fatty_acid!(C0 {}).unwrap()],
            //         true,
            //     )
            //     .unwrap()),
            // ]
            // .into_iter()
            // .chain(
            //     schema
            //         .iter_names()
            //         .filter(|&name| name != LABEL && name != FATTY_ACID)
            //         .map(|name| {
            //             let mean = index(name);
            //             let standard_deviation = lit(0.0).alias("StandardDeviation");
            //             let array = concat_arr(vec![lit(0.0)]).unwrap().alias("Array");
            //             as_struct(vec![
            //                 mean.alias("Mean"),
            //                 standard_deviation.alias("StandardDeviation"),
            //                 array.alias("Array"),
            //             ])
            //             .alias(name.clone())
            //         }),
            // )
            // .collect::<Vec<_>>();
        }
    }
    Ok(lazy_frame)
}

/// Filter
fn filter(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    match key.filter {
        Filter::Intersection => {
            // Значения отличные от нуля присутствуют во всех столбцах (AND)
            lazy_frame = lazy_frame.filter(all_horizontal([all()
                .exclude_cols([LABEL, FATTY_ACID])
                .as_expr()
                .is_not_null()])?);
        }
        Filter::Union => {
            // Значения отличные от нуля присутствуют в одном или более столбцах (OR)
            lazy_frame = lazy_frame.filter(any_horizontal([all()
                .exclude_cols([LABEL, FATTY_ACID])
                .as_expr()
                .is_not_null()])?);
        }
        Filter::Difference => {
            // Значения отличные от нуля отсутствуют в одном или более столбцах (XOR)
            lazy_frame = lazy_frame.filter(any_horizontal([all()
                .exclude_cols([LABEL, FATTY_ACID])
                .as_expr()
                .is_null()])?);
        }
    }
    // Threshold
    // Значение в одном или более столбцах больше threshold
    lazy_frame = lazy_frame.filter(any_horizontal([all()
        .exclude_cols([LABEL, FATTY_ACID])
        .as_expr()
        .struct_()
        .field_by_name("Mean")
        .gt(key.threshold.into_inner())])?);
    Ok(lazy_frame)
}

/// Sort
fn sort(mut lazy_frame: LazyFrame, key: Key) -> LazyFrame {
    match key.sort {
        Sort::Key => {
            lazy_frame = lazy_frame.sort_by_exprs(
                [
                    col(FATTY_ACID).fatty_acid().carbon(),
                    col(FATTY_ACID).fatty_acid().double_bounds_unsaturation(),
                    col(FATTY_ACID).fatty_acid().indices(),
                    col(LABEL),
                    all().exclude_cols([LABEL, FATTY_ACID]).as_expr(),
                ],
                SortMultipleOptions::new(),
            );
        }
        Sort::Value => {
            lazy_frame = lazy_frame.sort_by_exprs(
                [all().exclude_cols([LABEL, FATTY_ACID]).as_expr()],
                SortMultipleOptions::new()
                    .with_order_descending(true)
                    .with_nulls_last(true),
            );
        }
    }
    lazy_frame
}

pub(crate) mod format;
pub(crate) mod indices;
pub(crate) mod metrics;
