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

use crate::configuration::pole_figure_configuration::PoleFiguresConfiguration;
use serde_derive::Deserialize;
/// Global configuration file containing the information on where to find, read and how to analyze the CPO data.
#[derive(Deserialize)]
pub struct Config {
    /// The location which is the basis for all other paths.
    pub base_dir: String,
    /// A vector containing the directories which contains the data and where results are written to.
    pub experiment_dirs: Vec<String>,
    /// Pole figure configuration options.
    pub pole_figures: Option<PoleFiguresConfiguration>,
    /// Whether the CPO data has been compressed.
    pub compressed: bool,
}
