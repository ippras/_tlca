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
        // Unnormalized
        let unnormalized = key.frame.data.clone().lazy().select([
            nth(1).fill_null(0).alias(LEFT),
            nth(2).fill_null(0).alias(RIGHT),
        ]);
        let data_frame = unnormalized
            .clone()
            .select([
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
            ])
            .collect()?;
        // Normalized
        let m = || (col(LEFT) + col(RIGHT)) / lit(2);
        let kld = |left: Expr, rigth| (left.clone() * (left / rigth).log(E)).fill_nan(0).sum();
        let jsd = || (lit(0.5) * (kld(col(LEFT), m()) + kld(col(RIGHT), m())));
        let lazy_frame = unnormalized
            .select([
                (col(LEFT) / col(LEFT).sum()),
                (col(RIGHT) / col(RIGHT).sum()),
            ])
            .select([
                as_struct(vec![
                    kld(col(LEFT), col(RIGHT)).alias("LeftRight"),
                    kld(col(RIGHT), col(LEFT)).alias("RightLeft"),
                ])
                .alias("KullbackLeiblerDivergence"),
                jsd().sqrt().alias("JensenShannonDistance"),
            ]);
        data_frame.hstack(lazy_frame.collect()?.get_columns())
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
    pub(crate) frame: &'a Hashed<MetaDataFrame>,
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
