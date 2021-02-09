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

use crate::pole_figures::crystal_axis::CrystalAxes;
use crate::pole_figures::minerals::Mineral;

use ndarray::Array2;

/// Stores the information related to a single pole figure.
#[derive(Clone)]
pub struct PoleFigure {
    pub mineral: Mineral,
    pub crystal_axis: CrystalAxes,
    pub counts: Array2<f64>,
    pub max_count: f64,
}
