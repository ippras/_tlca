use crate::{
    app::states::fatty_acids::{Indices, Settings, StereospecificNumbers},
    utils::HashedDataFrame,
};
use egui::util::cache::{ComputerMut, FrameCache};
use lipid::prelude::*;
use polars::prelude::*;
use std::{iter::once, num::NonZeroI8};
use tracing::instrument;

/// Indices computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Indices computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    #[instrument(skip(self), err)]
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

/// Indices key
#[derive(Clone, Copy, Debug, Hash)]
pub(crate) struct Key<'a> {
    pub(crate) frame: &'a HashedDataFrame,
    pub(crate) indices: &'a Indices,
    pub(crate) stereospecific_numbers: StereospecificNumbers,
}

impl<'a> Key<'a> {
    pub(crate) fn new(frame: &'a HashedDataFrame, settings: &'a Settings) -> Self {
        Self {
            frame,
            indices: &settings.parameters.indices,
            stereospecific_numbers: settings.parameters.stereospecific_numbers,
        }
    }
}

/// Indices value
type Value = DataFrame;

fn compute(key: Key) -> PolarsResult<LazyFrame> {
    let mut lazy_frame = key.frame.data_frame.clone().lazy();
    let schema = lazy_frame.collect_schema()?;
    let lazy_frames = key
        .indices
        .iter()
        .filter(|index| index.visible)
        .map(|index| {
            let id = lit(Series::new(
                PlSmallStr::from_static("Index"),
                [index.name.clone()],
            ));
            let values = schema
                .iter_names()
                .filter(|&name| name != LABEL && name != FATTY_ACID)
                .map(|name| {
                    let expr = col(name.clone()).struct_().field_by_name("Mean");
                    let mean = compute_index(&index.name, expr);
                    let standard_deviation = lit(0.0).alias("StandardDeviation");
                    let array = concat_arr(vec![lit(0.0)]).unwrap().alias("Array");
                    as_struct(vec![
                        mean.alias("Mean"),
                        standard_deviation.alias("StandardDeviation"),
                        array.alias("Array"),
                    ])
                    .alias(name.clone())
                });
            let exprs = once(id).chain(values).collect::<Vec<_>>();
            lazy_frame.clone().select(exprs)
        })
        .collect::<Vec<_>>();
    lazy_frame = concat(lazy_frames, Default::default())?;
    Ok(lazy_frame)
}

fn compute_index(name: &str, expr: Expr) -> Expr {
    match name {
        "Saturated" => col(FATTY_ACID).fatty_acid().saturated(expr),
        "Monounsaturated" => col(FATTY_ACID).fatty_acid().monounsaturated(expr),
        "Polyunsaturated" => col(FATTY_ACID).fatty_acid().polyunsaturated(expr),
        "Unsaturated" => col(FATTY_ACID).fatty_acid().unsaturated(expr, None),
        "Unsaturated-9" => col(FATTY_ACID)
            .fatty_acid()
            .unsaturated(expr, NonZeroI8::new(-9)),
        "Unsaturated-6" => col(FATTY_ACID)
            .fatty_acid()
            .unsaturated(expr, NonZeroI8::new(-6)),
        "Unsaturated-3" => col(FATTY_ACID)
            .fatty_acid()
            .unsaturated(expr, NonZeroI8::new(-3)),
        "Unsaturated9" => col(FATTY_ACID)
            .fatty_acid()
            .unsaturated(expr, NonZeroI8::new(9)),
        "Trans" => col(FATTY_ACID).fatty_acid().trans(expr),
        "EicosapentaenoicAndDocosahexaenoic" => col(FATTY_ACID)
            .fatty_acid()
            .eicosapentaenoic_and_docosahexaenoic(expr),
        "FishLipidQuality" => col(FATTY_ACID).fatty_acid().fish_lipid_quality(expr),
        "HealthPromotingIndex" => col(FATTY_ACID).fatty_acid().health_promoting_index(expr),
        "HypocholesterolemicToHypercholesterolemic" => col(FATTY_ACID)
            .fatty_acid()
            .hypocholesterolemic_to_hypercholesterolemic(expr),
        "IndexOfAtherogenicity" => col(FATTY_ACID).fatty_acid().index_of_atherogenicity(expr),
        "IndexOfThrombogenicity" => col(FATTY_ACID).fatty_acid().index_of_thrombogenicity(expr),
        "LinoleicToAlphaLinolenic" => col(FATTY_ACID)
            .fatty_acid()
            .linoleic_to_alpha_linolenic(expr),
        "Polyunsaturated-6ToPolyunsaturated-3" => col(FATTY_ACID)
            .fatty_acid()
            .polyunsaturated_6_to_polyunsaturated_3(expr),
        "PolyunsaturatedToSaturated" => col(FATTY_ACID)
            .fatty_acid()
            .polyunsaturated_to_saturated(expr),
        "UnsaturationIndex" => col(FATTY_ACID).fatty_acid().unsaturation_index(expr),
        _ => unreachable!(),
    }
}
