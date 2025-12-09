use crate::{
    app::states::fatty_acids::{Factor, Settings},
    r#const::{FILTER, MEAN, SAMPLE, STANDARD_DEVIATION},
    utils::{HashedDataFrame, polars::sum_arr},
};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use polars::prelude::*;

/// Factors computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Factors computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        let lazy_frame = compute(key)?;
        let data_frame = lazy_frame.collect()?;
        Ok(data_frame)
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Factors key
#[derive(Clone, Copy, Debug, Hash)]
pub(crate) struct Key<'a> {
    pub(crate) frame: &'a HashedDataFrame,
    pub(crate) ddof: u8,
    pub(crate) factor: Factor,
    pub(crate) normalize_factor: bool,
    pub(crate) percent: bool,
    pub(crate) precision: usize,
    pub(crate) significant: bool,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frame: &'a HashedDataFrame, settings: &Settings) -> Self {
        Self {
            frame,
            ddof: 1,
            factor: settings.factor,
            normalize_factor: settings.normalize_factor,
            percent: settings.percent,
            precision: settings.precision,
            significant: settings.significant,
        }
    }
}

/// Factors value
type Value = DataFrame;

fn compute(key: Key) -> PolarsResult<LazyFrame> {
    let mut lazy_frame = key.frame.data_frame.clone().lazy();
    println!(
        "FF0: {}",
        lazy_frame
            .clone()
            .select([col("К-2233.2025-10-29").struct_().field_by_name("*")])
            // .unnest(cols(["К-2233.2025-10-29"]), None)
            .collect()?
    );
    let stereospecific_numbers = |name: &PlSmallStr| {
        (
            col(name.clone())
                .struct_()
                .field_by_name(STEREOSPECIFIC_NUMBERS123),
            col(name.clone())
                .struct_()
                .field_by_name(STEREOSPECIFIC_NUMBERS2),
        )
    };
    let schema = lazy_frame.collect_schema()?;
    let exprs = schema
        .iter_names()
        .filter(|name| !matches!(name.as_str(), LABEL | FATTY_ACID | FILTER))
        .map(|name| {
            let (sn123, sn2) = stereospecific_numbers(name);
            let tag = sn123.struct_().field_by_name("Array");
            let mag2 = sn2.struct_().field_by_name("Array");
            let mut factor = match key.factor {
                Factor::Selectivity => {
                    let is_unsaturated = col(FATTY_ACID).fatty_acid().is_unsaturated(None);
                    let unsaturated_mag2 = sum_arr(mag2.clone().filter(is_unsaturated.clone()))?;
                    let unsaturated_tag = sum_arr(tag.clone().filter(is_unsaturated))?;
                    (mag2 * unsaturated_tag) / (tag * unsaturated_mag2)
                    // col(FATTY_ACID).fatty_acid().selectivity_factor(mag2, tag)
                }
                Factor::Enrichment => FattyAcidExpr::enrichment_factor(mag2, tag),
            };
            if key.normalize_factor {
                factor = factor / lit(3);
            }
            Ok(as_struct(vec![
                factor.clone().arr().mean().alias(MEAN),
                factor.clone().arr().std(key.ddof).alias(STANDARD_DEVIATION),
                factor.alias(SAMPLE),
            ])
            .alias(name.clone()))
        })
        .collect::<PolarsResult<Vec<_>>>()?;
    lazy_frame = lazy_frame.with_columns(exprs);
    println!(
        "FF1: {:?}",
        lazy_frame
            .clone()
            .select([nth(2).as_expr()])
            .unnest(all(), None)
            .collect()
            .unwrap()
    );
    Ok(lazy_frame)
}
