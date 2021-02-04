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
    pub pole_figures: PoleFiguresConfiguration,
    /// Whether the CPO data has been compressed.
    pub compressed: bool,
}
