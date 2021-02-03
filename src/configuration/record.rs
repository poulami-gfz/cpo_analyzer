use serde_derive::Deserialize;
/// A structure to load the CPO data
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Record {
    pub id: usize,
    pub mineral_0_EA_phi: Option<f64>,
    pub mineral_0_EA_theta: Option<f64>,
    pub mineral_0_EA_z: Option<f64>,
    pub mineral_1_EA_phi: Option<f64>,
    pub mineral_1_EA_theta: Option<f64>,
    pub mineral_1_EA_z: Option<f64>,
}
