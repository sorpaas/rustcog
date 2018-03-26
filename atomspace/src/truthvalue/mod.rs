pub type Strength = f64;
pub type Confidence = f64;
pub type Count = f64;

pub enum TruthValue {
    Simple(SimpleTruthValue),
    Count(CountTruthValue),
    Evidence(EvidenceTruthValue),
    Indefinite(IndefiniteTruthValue),
    Fuzzy(FuzzyTruthValue),
    Probabilistic(ProbabilisticTruthValue),
}

/// A TruthValue that stores a strength and confidence.
pub struct SimpleTruthValue {
    /// Mean of the strength of the TV over all observations.
    pub strength: Strength,
    /// Estimate of confidence of the observation.
    pub confidence: Confidence,
}

/// A TruthValue that stores a strength, a confidence and the
/// number of observations.
pub struct CountTruthValue {
    /// Mean of the strength of the TV over all observations.
    pub strength: Strength,
    /// Estimate of confidence of the observation.
    pub confidence: Confidence,
    /// Raw count.
    pub count: Count,
}

/// A TruthValue that stores count of positives and totals.
pub struct EvidenceTruthValue {
    /// Positive evidence count, i.e. number of observations
    /// corroborating with the atom's truth.
    pub positive: Count,
    /// Total number of observations.
    pub total: Count,
}

/// Indefinite probabilities are in the form ([L,U],b,N). In
/// practical work, N will be hold constant and thus we have only
/// ([L,U],b).
pub struct IndefiniteTruthValue {
    /// Lower bound L.
    pub lower: Strength,
    /// Upper bound U.
    pub upper: Strength,
    /// Confidence b.
    pub confidence: Confidence,
}

/// A TruthValue that stores a mean and the number of observations
/// (strength and confidence).
pub struct FuzzyTruthValue {
    /// Mean of the strength of the TV over all observations.
    pub strength: Strength,
    /// Total number of observations used to estimate the mean.
    pub count: Count,
}

/// A TruthValue that stores a strength, a confidence and the
/// number of observations.
pub struct ProbabilisticTruthValue {
    /// Mean of the strength of the TV over all observations.
    pub strength: Strength,
    /// Estimate of confidence of the observation.
    pub confidence: Confidence,
    /// Raw count.
    pub count: Count,
}
