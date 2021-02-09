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

use cpo_analyzer::load_configuration_file;
use cpo_analyzer::process_configuration;

use assert_cmd::prelude::*;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_library_run() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = PathBuf::from("examples/example_experiment_1/example_1_config.toml");

    let mut configuration =
        load_configuration_file(config_file).expect("Could not load configuration file");
    configuration
        .pole_figures
        .as_mut()
        .unwrap()
        .figure_output_dir = "test_results_library/".to_string();
    process_configuration(configuration).unwrap();

    // test whether the command works by comparing the same image.
    let cmd = Command::new("compare")
    .arg("-metric")
    .arg("AE")
    .arg("examples/example_experiment_1/CPO_figures/weighted_LPO_elastic_oli_ens_A-B-C-Axis_Batlow_g1_sp301_t00001.00000.png")
    .arg("examples/example_experiment_1/CPO_figures/weighted_LPO_elastic_oli_ens_A-B-C-Axis_Batlow_g1_sp301_t00001.00000.png")
    .arg("null")
    .ok();
    if cmd.is_ok() {
        Command::new("compare")
        .arg("-metric")
        .arg("AE")
        .arg("examples/example_experiment_1/CPO_figures/weighted_LPO_elastic_oli_ens_A-B-C-Axis_Batlow_g1_sp301_t00001.00000.png")
        .arg("examples/example_experiment_1/test_results_binary/weighted_LPO_elastic_oli_ens_A-B-C-Axis_Batlow_g1_sp301_t00001.00000.png")
        .arg("null")
        .assert()
        .success();
    } else {
        println!("Program compare from magick not found. Test did not check the output.");
    };

    Ok(())
}

#[test]
fn test_binary() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cpo_analyzer")?;
    cmd.arg("tests/test_example_1.toml");
    cmd.assert().success();

    // test whether the command works by comparing the same image.
    let cmd = Command::new("compare")
    .arg("-metric")
    .arg("AE")
    .arg("examples/example_experiment_1/CPO_figures/weighted_LPO_elastic_oli_ens_A-B-C-Axis_Batlow_g1_sp301_t00001.00000.png")
    .arg("examples/example_experiment_1/CPO_figures/weighted_LPO_elastic_oli_ens_A-B-C-Axis_Batlow_g1_sp301_t00001.00000.png")
    .arg("null")
    .ok();
    if cmd.is_ok() {
        Command::new("compare")
        .arg("-metric")
        .arg("AE")
        .arg("examples/example_experiment_1/CPO_figures/weighted_LPO_elastic_oli_ens_A-B-C-Axis_Batlow_g1_sp301_t00001.00000.png")
        .arg("examples/example_experiment_1/test_results_binary/weighted_LPO_elastic_oli_ens_A-B-C-Axis_Batlow_g1_sp301_t00001.00000.png")
        .arg("null")
        .assert()
        .success();
    } else {
        println!("Program compare from magick not found. Test did not check the output.");
    };

    Ok(())
}
