use std::sync::LazyLock;

use crate::{
    app::{
        computers::matches_schema,
        states::fatty_acids::settings::{Settings, Sort, StereospecificNumbers, Threshold},
    },
    r#const::{MAJOR, VALUE, VALUE_},
    utils::{HashedDataFrame, HashedMetaDataFrame},
};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use polars::prelude::*;
use polars_ext::prelude::*;
use tracing::instrument;

/// Fatty acids computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Fatty acids computer
#[derive(Default)]
pub(crate) struct Computer;

/// Input schema
pub(crate) const INPUT_SCHEMA: LazyLock<SchemaRef> = LazyLock::new(|| {
    Arc::new(Schema::from_iter([
        Field::new(PlSmallStr::from_static(LABEL), DataType::String),
        field!(FATTY_ACID),
        Field::new(
            PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS123),
            DataType::Array(Box::new(DataType::Float64), 0),
        ),
        Field::new(
            PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS13),
            DataType::Array(Box::new(DataType::Float64), 0),
        ),
        Field::new(
            PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS2),
            DataType::Array(Box::new(DataType::Float64), 0),
        ),
    ]))
});

impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        if key.frames.is_empty() {
            return Ok(HashedDataFrame::EMPTY);
        }
        for frame in key.frames {
            matches_schema(&frame.data.data_frame, &INPUT_SCHEMA)?;
        }
        let mut lazy_frame = join(key)?;
        println!("lazy_frame: {}", lazy_frame.clone().collect().unwrap());
        // lazy_frame = value(lazy_frame);
        // println!("values: {}", lazy_frame.clone().collect().unwrap());
        lazy_frame = keep(lazy_frame, key)?;
        println!("filter: {}", lazy_frame.clone().collect().unwrap());
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
    pub(crate) ddof: u8,
    pub(crate) percent: bool,
    pub(crate) precision: usize,
    pub(crate) significant: bool,
    pub(crate) sort: Option<Sort>,
    pub(crate) stereospecific_numbers: StereospecificNumbers,
    pub(crate) threshold: &'a Threshold,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frames: &'a [HashedMetaDataFrame], settings: &'a Settings) -> Self {
        Self {
            frames,
            ddof: 1,
            percent: settings.percent,
            precision: settings.precision,
            significant: settings.significant,
            sort: settings.sort,
            stereospecific_numbers: settings.stereospecific_numbers,
            threshold: &settings.threshold,
        }
    }
}

/// Fatty acids value
type Value = HashedDataFrame;

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
            .alias(format!("{VALUE}_{}", frame.meta.format("."))),
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

// /// Values
// fn values(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
//     let schema = lazy_frame.collect_schema()?;
//     let exprs = schema
//         .iter_names()
//         .filter(|name| !matches!(name.as_str(), LABEL | FATTY_ACID))
//         .map(|name| {
//             // let field = |stereospecific_numbers: &str| {
//             //     let expr = col(name.as_str())
//             //         .struct_()
//             //         .field_by_name(stereospecific_numbers);
//             //     let mean = expr.clone().arr().mean();
//             //     // TODO: DDOF
//             //     let standard_deviation = expr.clone().arr().std(1);
//             //     ternary_expr(
//             //         mean.clone().neq(0),
//             //         as_struct(vec![
//             //             mean.alias(MEAN),
//             //             standard_deviation.alias(STANDARD_DEVIATION),
//             //             expr.alias(SAMPLE),
//             //         ]),
//             //         lit(NULL),
//             //     )
//             //     .alias(stereospecific_numbers)
//             // };
//             // as_struct(vec![
//             //     field(STEREOSPECIFIC_NUMBERS123),
//             //     field(STEREOSPECIFIC_NUMBERS13),
//             //     field(STEREOSPECIFIC_NUMBERS2),
//             // ])
//             // .alias(name.clone())
//             as_struct(vec![
//                 Array::builder()
//                     .expr(
//                         col(name.as_str())
//                             .struct_()
//                             .field_by_name(STEREOSPECIFIC_NUMBERS123),
//                     )
//                     .ddof(key.ddof)
//                     .percent(key.percent)
//                     .precision(key.precision)
//                     .significant(key.significant)
//                     .build(),
//                 Array::builder()
//                     .expr(
//                         col(name.as_str())
//                             .struct_()
//                             .field_by_name(STEREOSPECIFIC_NUMBERS13),
//                     )
//                     .ddof(key.ddof)
//                     .percent(key.percent)
//                     .precision(key.precision)
//                     .significant(key.significant)
//                     .build(),
//                 Array::builder()
//                     .expr(
//                         col(name.as_str())
//                             .struct_()
//                             .field_by_name(STEREOSPECIFIC_NUMBERS2),
//                     )
//                     .ddof(key.ddof)
//                     .percent(key.percent)
//                     .precision(key.precision)
//                     .significant(key.significant)
//                     .build(),
//             ])
//             .alias(name.clone())
//         })
//         .collect::<Vec<_>>();
//     lazy_frame = lazy_frame.with_columns(exprs);
//     Ok(lazy_frame)
// }
// /// Value
// fn value(mut lazy_frame: LazyFrame) -> LazyFrame {
//     lazy_frame = lazy_frame.select([
//         col(LABEL),
//         col(FATTY_ACID),
//         as_struct(vec![col(VALUE_)]).alias(VALUE),
//     ]);
//     lazy_frame
// }

/// Keep
fn keep(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    // Берем среднее значение массива, так как иначе пришлось бы сравнивать все повторности попарно
    // Значение в любом из столбцов больше threshold
    let field = |name| {
        any_horizontal([col(VALUE_)
            .struct_()
            .field_by_name(name)
            .arr()
            .mean()
            .fill_null(0)
            .gt_eq(key.threshold.auto.0)])
    };
    lazy_frame = lazy_frame.with_column(
        as_struct(vec![
            field(STEREOSPECIFIC_NUMBERS123)?,
            field(STEREOSPECIFIC_NUMBERS13)?,
            field(STEREOSPECIFIC_NUMBERS2)?,
        ])
        .alias(MAJOR),
    );

    // if key.threshold.filter {
    //     lazy_frame = lazy_frame.filter(col(FILTER));
    // }
    // if key.threshold.sort {
    //     lazy_frame = lazy_frame.sort(
    //         [FILTER],
    //         SortMultipleOptions::new()
    //             .with_maintain_order(true)
    //             .with_order_descending(true),
    //     );
    // }
    Ok(lazy_frame)
}

/// Sort
fn sort(mut lazy_frame: LazyFrame, key: Key) -> LazyFrame {
    if let Some(sort) = key.sort {
        match sort {
            Sort::Key => {
                lazy_frame = lazy_frame.sort_by_exprs(
                    [
                        col(FATTY_ACID).fatty_acid().carbon(),
                        col(FATTY_ACID).fatty_acid().double_bounds_unsaturation(),
                        col(FATTY_ACID).fatty_acid().indices(),
                        col(LABEL),
                    ],
                    SortMultipleOptions::new().with_maintain_order(true),
                );
            }
            Sort::Value => {
                lazy_frame = lazy_frame.sort_by_exprs(
                    [all().exclude_cols([LABEL, FATTY_ACID]).as_expr()],
                    SortMultipleOptions::new()
                        .with_maintain_order(true)
                        .with_order_descending(true)
                        .with_nulls_last(true),
                );
            }
        }
    }
    lazy_frame
}

// /// Filter
// fn filter(mut lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
//     let expr = all().exclude_cols([LABEL, FATTY_ACID]).as_expr();
//     // Join
//     let mut predicate = match key.filter {
//         Filter::Intersection => {
//             // Значения отличные от нуля присутствуют во всех столбцах (AND)
//             all_horizontal([expr.clone().is_not_null()])?
//         }
//         Filter::Union => {
//             // Значения отличные от нуля присутствуют в одном или более столбцах (OR)
//             any_horizontal([expr.clone().is_not_null()])?
//         }
//         Filter::Difference => {
//             // Значения отличные от нуля отсутствуют в одном или более столбцах (XOR)
//             any_horizontal([expr.clone().is_null()])?
//         }
//     };
//     lazy_frame = lazy_frame.filter(predicate);
//     // Threshold
//     // Значение в одном или более столбцах больше threshold
//     predicate = any_horizontal([expr
//         .clone()
//         .struct_()
//         .field_by_name(MEAN)
//         .gt_eq(key.threshold.0)
//         .and(expr.is_not_null())])?;
//     lazy_frame = lazy_frame.with_column(predicate.alias("Filter"));
//     Ok(lazy_frame)
// }
