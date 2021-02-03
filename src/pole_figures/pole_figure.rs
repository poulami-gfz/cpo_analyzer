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
