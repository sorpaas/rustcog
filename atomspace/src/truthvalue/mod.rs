mod simple;

pub use self::simple::SimpleTruthValue;

pub type Strength = f64;
pub type Confidence = f64;
pub type Count = f64;

pub trait TruthValue {
    fn mean(&self) -> Strength;
    fn confidence(&self) -> Confidence;
    fn count(&self) -> Count;
}
