#![rustfmt::skip]

pub const METRICS: &str = include_str!("../doc/ru/Metrics.md");

// Correlation
pub const PEARSON_CORRELATION_COEFFICIENT: &str = include_str!("../doc/ru/Correlation/PearsonCorrelation.md");
pub const SPEARMAN_RANK_CORRELATION_COEFFICIENT: &str = include_str!("../doc/ru/Correlation/SpearmanCorrelation.md");
// Similarity between two points
pub const CHEBYSHEV_DISTANCE: &str = include_str!("../doc/ru/Similarity/BetweenTwoPoints/Chebyshev.md");
pub const EUCLIDEAN_DISTANCE: &str = include_str!("../doc/ru/Similarity/BetweenTwoPoints/Euclidean.md");
pub const MANHATTAN_DISTANCE: &str = include_str!("../doc/ru/Similarity/BetweenTwoPoints/Manhattan.md");
pub const CANBERRA_DISTANCE: &str = include_str!("../doc/ru/Similarity/BetweenTwoPoints/Canberra.md");
pub const MINKOWSKI_DISTANCE: &str = include_str!("../doc/ru/Similarity/BetweenTwoPoints/Minkowski.md");
// Similarity between two sets
pub const BRAUN_BLANQUET_COEFFICIENT: &str = include_str!("../doc/ru/Similarity/BetweenTwoSets/Braun-Blanquet.md");
pub const COSINE_COEFFICIENT: &str = include_str!("../doc/ru/Similarity/BetweenTwoSets/Cosine.md");
pub const JACCARD_COEFFICIENT: &str = include_str!("../doc/ru/Similarity/BetweenTwoSets/Jaccard.md");
pub const KULCZYNSKI_COEFFICIENT: &str = include_str!("../doc/ru/Similarity/BetweenTwoSets/Kulczynski.md");
pub const OVERLAP_COEFFICIENT: &str = include_str!("../doc/ru/Similarity/BetweenTwoSets/Overlap.md");
pub const SØRENSEN_COEFFICIENT: &str = include_str!("../doc/ru/Similarity/BetweenTwoSets/Sørensen.md");
// Similarity between two discrete probability distributions
pub const BHATTACHARYYA_COEFFICIENT: &str = include_str!("../doc/ru/Similarity/BetweenTwoDiscreteProbabilityDistributions/Bhattacharyya.md");
pub const HELLINGER_COEFFICIENT: &str = include_str!("../doc/ru/Similarity/BetweenTwoDiscreteProbabilityDistributions/Hellinger.md");
pub const JENSEN_SHANNON_COEFFICIENT: &str = include_str!("../doc/ru/Similarity/BetweenTwoDiscreteProbabilityDistributions/JensenShannon.md");

// pub const BRAY_CURTIS_DISSIMILARITY: &str = include_str!("../doc/metrics/ru/BrayCurtisDissimilarity.md");
// pub const KULLBACK_LEIBLER_DIVERGruCE: &str = include_str!("../doc/metrics/ru/KullbackLeiblerDivergruce.md");
// pub const WASSERSTEIN_DISTANCE: &str = include_str!("../doc/metrics/ru/WassersteinDistance.md");
