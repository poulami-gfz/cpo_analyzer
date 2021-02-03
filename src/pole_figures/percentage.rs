/// A structure which holds a total value of which can be used to compute a percentage.
pub struct Percentage {
    pub total: f64,
}

impl Percentage {
    /// Compute a percentage of the total value stored in the structure.
    pub fn calc(&self, percentage: f64) -> f64 {
        (percentage / 100.) * self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_strucutre() {
        let percentage = Percentage { total: 11.5 };
        assert_eq!(percentage.total, 11.5);
        assert_eq!(percentage.calc(50.), 5.75);
    }
}
