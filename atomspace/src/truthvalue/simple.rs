use super::{Strength, Confidence, Count, TruthValue};

const DEFAULT_K: Confidence = 800.0;

pub struct SimpleTruthValue {
    mean: Strength,
    confidence: Confidence,
}

impl SimpleTruthValue {
    pub fn new(mean: Strength, confidence: Confidence) -> Self {
        Self { mean, confidence }
    }
}

impl TruthValue for SimpleTruthValue {
    fn mean(&self) -> Strength {
        self.mean
    }

    fn confidence(&self) -> Confidence {
        self.confidence
    }

    fn count(&self) -> Count {
        let cf = if self.confidence >= 0.9999998 {
            0.9999998
        } else {
            self.confidence
        };
        DEFAULT_K * cf / (1.0 - cf)
    }
}
