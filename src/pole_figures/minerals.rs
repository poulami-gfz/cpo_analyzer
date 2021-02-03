use serde_derive::Deserialize;
/// A enum to define what mineral of the pole figure should be plotted. 
#[derive(Deserialize, Clone)]
pub enum Mineral {
    Olivine,
    Enstatite,
}
