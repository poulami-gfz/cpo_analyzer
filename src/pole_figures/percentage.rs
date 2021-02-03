/// A structure which holds a total value of which can be used to compute a percentage.
pub struct Percentage {
    pub total: f64,
}

impl Percentage {
    /// Compute a percentage of the provided value `part` of the total value stored in the structure.
    pub fn calc(&self, part: f64) -> f64 {
        (part / 100.) * self.total
    }
}
