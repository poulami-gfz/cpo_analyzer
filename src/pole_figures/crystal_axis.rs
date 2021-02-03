use serde_derive::{Deserialize, Serialize};

/// A enum to define what axis of the pole figure should be plotted. 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CrystalAxes {
    AAxis,
    BAxis,
    CAxis,
}
