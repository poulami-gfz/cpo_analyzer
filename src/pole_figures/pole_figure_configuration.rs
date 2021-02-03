use crate::pole_figures::{crystal_axis::CrystalAxes, minerals::Mineral};

use serde_derive::Deserialize;
/// The configuration of the pole figure analysis.
#[derive(Deserialize)]
pub struct PoleFiguresConfiguration {
    /// Whether to inluce elasticity information in the header of the polefigure.
    pub elastisity_header: bool,
    /// A vector containing the times at which to make the pole figures. The acutal times are set to the closest time 
    /// for which data is available.
    pub times: Vec<f64>,
    /// A vector containing the id's of the particles to plot. A plot for is made for every id.
    pub particle_ids: Vec<usize>,
    /// A vector of the crytal axis to plot. This will be plot on the horizontal axis of the plot, and the maximum 
    /// of the maximum count of the pole figures will be used as the new maximum to scale the colors. Available options 
    /// are `AAxis`, `BAxis` and `CAxis`. 
    pub axes: Vec<CrystalAxes>,
    /// A vector containing the minerals to be plot. These will be added as a vertical axis to the plot. Available 
    /// options are `Olivine` and `Enstatite`.
    pub minerals: Vec<Mineral>,
}
