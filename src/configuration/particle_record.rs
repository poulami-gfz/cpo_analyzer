use serde_derive::Deserialize;
/// A structure to hold the particle data, including the id, position, deformation type, and optionally elasticity information.
#[derive(Debug, Deserialize)]
pub struct ParticleRecord {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub olivine_deformation_type: f64,
    pub full_norm_square: Option<f64>,
    pub triclinic_norm_square_p1: Option<f64>,
    pub triclinic_norm_square_p2: Option<f64>,
    pub triclinic_norm_square_p3: Option<f64>,
    pub monoclinic_norm_square_p1: Option<f64>,
    pub monoclinic_norm_square_p2: Option<f64>,
    pub monoclinic_norm_square_p3: Option<f64>,
    pub orthohombic_norm_square_p1: Option<f64>,
    pub orthohombic_norm_square_p2: Option<f64>,
    pub orthohombic_norm_square_p3: Option<f64>,
    pub tetragonal_norm_square_p1: Option<f64>,
    pub tetragonal_norm_square_p2: Option<f64>,
    pub tetragonal_norm_square_p3: Option<f64>,
    pub hexagonal_norm_square_p1: Option<f64>,
    pub hexagonal_norm_square_p2: Option<f64>,
    pub hexagonal_norm_square_p3: Option<f64>,
    pub isotropic_norm_square: Option<f64>,
}
