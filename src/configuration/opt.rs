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

use std::path::PathBuf;
use structopt::StructOpt;

/// A structure to collect the commandline arugments.
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    /// Files to process
    #[structopt(name = "config file", parse(from_os_str))]
    pub config_file: PathBuf,
}
