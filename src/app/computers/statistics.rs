use crate::{app::panes::calculation::settings::Settings, utils::Hashed};
use egui::util::cache::{ComputerMut, FrameCache};
use metadata::MetaDataFrame;
use polars::{
    error::PolarsResult,
    lazy::dsl::{max_horizontal, min_horizontal},
    prelude::*,
};
use std::{
    f64::consts::E,
    hash::{Hash, Hasher},
};
use tracing::instrument;

const LEFT: &str = "Left";
const RIGHT: &str = "Right";

/// Statistics computed
pub(crate) type Computed = FrameCache<Value, Computer>;

/// Statistics computer
#[derive(Default)]
pub(crate) struct Computer;

impl Computer {
    #[instrument(skip(self), err)]
    fn try_compute(&mut self, key: Key) -> PolarsResult<DataFrame> {
        let m = || (col(LEFT) + col(RIGHT)) / lit(2);
        let kl = |left| (col(left) * (col(left) / m()).log(E)).fill_nan(0).sum();
        let jsd = || (lit(0.5) * (kl(LEFT) + kl(RIGHT)));

        let mut lazy_frame = key
            .frame
            .value
            .clone()
            .lazy()
            .select([
                nth(1).fill_null(0).alias(LEFT),
                nth(2).fill_null(0).alias(RIGHT),
            ])
            .with_columns([
                (col(LEFT) - col(RIGHT))
                    .pow(2)
                    .sum()
                    .sqrt()
                    .alias("EuclideanDistance"),
                (col(LEFT) - col(RIGHT)).max().alias("ChebyshevDistance"),
                (col(LEFT) - col(RIGHT))
                    .abs()
                    .sum()
                    .alias("ManhattanDistance"),
                (lit(1)
                    - (col(LEFT) * col(RIGHT)).sum()
                        / (col(LEFT).pow(2).sum().sqrt() * col(RIGHT).pow(2).sum().sqrt()))
                .alias("CosineDistance"),
                ((col(LEFT) - col(RIGHT)).abs().sum() / (col(LEFT) + col(RIGHT)).sum())
                    .alias("BrayCurtisDissimilarity"),
                (lit(1)
                    - min_horizontal([col(LEFT), col(RIGHT)])?.sum()
                        / max_horizontal([col(LEFT), col(RIGHT)])?.sum())
                .alias("RuzickaDistance"),
                pearson_corr(col(LEFT), col(RIGHT)).alias("PearsonCorrelation"),
                spearman_rank_corr(col(LEFT), col(RIGHT), false).alias("SpearmanCorrelation"),
            ]);
        lazy_frame = lazy_frame
            .with_columns([
                (col(LEFT) / col(LEFT).sum()),
                (col(RIGHT) / col(RIGHT).sum()),
            ])
            .with_columns([jsd().sqrt().alias("JensenShannonDistance")]);
        lazy_frame.collect()
    }
}

impl ComputerMut<Key<'_>, Value> for Computer {
    fn compute(&mut self, key: Key) -> Value {
        self.try_compute(key).unwrap()
    }
}

/// Statistics key
#[derive(Clone, Copy, Debug)]
pub(crate) struct Key<'a> {
    pub(crate) frame: &'a Hashed<DataFrame>,
    pub(crate) settings: &'a Settings,
}

impl Hash for Key<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.frame.hash(state);
        self.settings.kind.hash(state);
    }
}

/// Statistics value
type Value = DataFrame;
