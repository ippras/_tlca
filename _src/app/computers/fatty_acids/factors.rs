use crate::{
    app::states::fatty_acids::settings::{Factor, Settings, StereospecificNumbers},
    r#const::{MEAN, SAMPLE, STANDARD_DEVIATION, THRESHOLD},
    utils::{HashedDataFrame, polars::sum_arr},
};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use ordered_float::OrderedFloat;
use polars::prelude::*;
use polars_ext::prelude::*;

/// Factors computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Factors computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    fn try_compute(&mut self, key: Key) -> PolarsResult<Value> {
        let mut lazy_frame = key.frame.data_frame.clone().lazy();
        println!("Factors0: {}", lazy_frame.clone().collect()?);
        lazy_frame = compute(lazy_frame, key)?;
        println!("Factors1: {}", lazy_frame.clone().collect()?);
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
    pub(crate) stereospecific_numbers: StereospecificNumbers,
    pub(crate) threshold: OrderedFloat<f64>,
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
            stereospecific_numbers: settings.stereospecific_numbers,
            threshold: settings.threshold.auto,
        }
    }
}

/// Factors value
type Value = DataFrame;

fn compute(lazy_frame: LazyFrame, key: Key) -> PolarsResult<LazyFrame> {
    let names = key.frame.schema().iter_names();
    let mut exprs = Vec::with_capacity(names.len());
    for name in names.filter(|name| !matches!(name.as_str(), LABEL | FATTY_ACID | THRESHOLD)) {
        let expr = col(name.as_str());
        let tag = expr
            .clone()
            .struct_()
            .field_by_name(STEREOSPECIFIC_NUMBERS123)
            .struct_()
            .field_by_name(SAMPLE);
        let mag2 = expr
            .struct_()
            .field_by_name(STEREOSPECIFIC_NUMBERS2)
            .struct_()
            .field_by_name(SAMPLE);
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
        // .fill_null(concat_arr(vec![lit(0.0)])?);
        if key.normalize_factor {
            factor = factor / lit(3);
        }
        exprs.push(
            as_struct(vec![
                factor
                    .clone()
                    .arr()
                    .mean()
                    .percent(key.percent)
                    .precision(key.precision, key.significant)
                    .alias(MEAN),
                factor
                    .clone()
                    .arr()
                    .std(key.ddof)
                    .percent(key.percent)
                    .precision(key.precision + 1, key.significant)
                    .alias(STANDARD_DEVIATION),
                factor
                    .arr()
                    .eval(
                        element()
                            .percent(key.percent)
                            .precision(key.precision, key.significant),
                        false,
                    )
                    .alias(SAMPLE),
            ])
            .alias(name.clone()),
        );
    }
    // println!("FF1: {:?}", lazy_frame.clone().collect().unwrap());
    Ok(lazy_frame.with_columns(exprs))
}
