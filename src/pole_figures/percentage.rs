/*
  Copyright (C) 2021 by the authors of the CPO Analyzer code.

  This file is part of the CPO Analyzer.

  The CPO Analyzer is free software; you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation; either version 2, or (at your option)
  any later version.

  The CPO Analyzer is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with the CPO Analyzer; see the file LICENSE.  If not see
  <http://www.gnu.org/licenses/>.
*/

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
