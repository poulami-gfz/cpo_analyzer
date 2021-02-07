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
