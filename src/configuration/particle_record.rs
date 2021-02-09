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

use serde_derive::Deserialize;
/// A structure to hold the particle data, including the id, position, deformation type, and optionally elasticity information.
#[derive(Debug, Deserialize)]
pub struct ParticleRecord {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub olivine_deformation_type: Option<f64>,
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
