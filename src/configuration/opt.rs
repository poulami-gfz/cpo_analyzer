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
