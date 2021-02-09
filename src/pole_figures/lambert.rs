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

use ndarray::Array;
use ndarray::Axis;
use ndarray::Dim;
use ndarray::Zip;

/// A structure to hold the Lambert computation data.
#[derive(Debug)]
pub struct Lambert {
    pub x_plane: Array<f64, Dim<[usize; 2]>>,
    pub z_plane: Array<f64, Dim<[usize; 2]>>,
    pub r_plane: f64,
    pub x: Array<f64, Dim<[usize; 2]>>,
    pub y: Array<f64, Dim<[usize; 2]>>,
    pub z: Array<f64, Dim<[usize; 2]>>,
}

/// Create a grid of evenly spaced points for contouring pole figure
/// matlab-version of drex uses 151 x 151 points.
///
/// Note wikipedia has the transformation for the lambert equal area projection
/// for both X,Y --> x,y,z and x,y,z --> X,Y
/// so given the Pa directions (unit vectors, do R = 1 on a sphere) in x,y,z,
/// you can get X,Y on the lambert projection to plot these as a scatter plot
/// without ever converting the spherical coordinates
pub fn create_lambert_equal_area_gridpoint(
    sphere_points: usize,
    hemisphere: String,
) -> Result<Lambert, Box<dyn std::error::Error>> {
    // Create a grid of points at increasing radius in the X and Y direction
    // Use the coordinate X,Y,R to plot these points on the lambert projection
    let r_plane: f64 = 2.0_f64.sqrt(); // need this to get full sphere in Lambert projection)
    let x_plane = Array::linspace(-r_plane, r_plane, sphere_points);
    let y_plane = x_plane.clone();
    let (x_plane, mut z_plane) = create_meshgrid(&x_plane, &y_plane)?;
    z_plane.invert_axis(Axis(0));

    // map onto lambert projection, assumes r = 1?
    // added np.abs to avoid tiny negative numbers in sqrt
    // todo, turn into enum and matchs
    let mut x = Array::zeros([sphere_points, sphere_points]);
    let mut y = Array::zeros([sphere_points, sphere_points]);
    let mut z = Array::zeros([sphere_points, sphere_points]);
    let mut mag = Array::zeros([sphere_points, sphere_points]);

    Zip::from(&mut x)
        .and(&x_plane)
        .and(&z_plane)
        .par_apply(|a, &x_plane, &z_plane| {
            *a = if 1. - (x_plane * x_plane + z_plane * z_plane) / 4. > std::f64::EPSILON {
                ((1. - (x_plane * x_plane + z_plane * z_plane) / 4.).abs()).sqrt() * x_plane
            } else {
                0.
            };
        });

    if hemisphere == "lower" {
        Zip::from(&mut y)
            .and(&x_plane)
            .and(&z_plane)
            .par_apply(|a, &x_plane, &z_plane| {
                *a = -(1. - (x_plane * x_plane + z_plane * z_plane) / 2.);
            });
        Zip::from(&mut z)
            .and(&x_plane)
            .and(&z_plane)
            .par_apply(|a, &x_plane, &z_plane| {
                *a =
                    ((1. - (x_plane * x_plane + z_plane * z_plane) / 4.).abs()).sqrt() * (-z_plane);
            });
    } else if hemisphere == "upper" {
        Zip::from(&mut y)
            .and(&x_plane)
            .and(&z_plane)
            .par_apply(|a, &x_plane, &z_plane| {
                *a = 1. - (x_plane * x_plane + z_plane * z_plane) / 2.;
            });
        Zip::from(&mut z)
            .and(&x_plane)
            .and(&z_plane)
            .par_apply(|a, &x_plane, &z_plane| {
                *a = ((1. - (x_plane * x_plane + z_plane * z_plane) / 4.).abs()).sqrt() * z_plane;
            });
    };

    // ensure unit vectors
    // Use these values of x,y,z to calculate the gaussian weighting function for contouring
    Zip::from(&mut mag)
        .and(&x)
        .and(&y)
        .and(&z)
        .par_apply(|a, &x, &y, &z| {
            *a = (x * x + y * y + z * z).sqrt();
        });
    let mag = mag;
    x = x / &mag;
    y = y / &mag;
    z = z / &mag;

    Ok(Lambert {
        x_plane: x_plane,
        z_plane: z_plane,
        r_plane: r_plane,
        x: x,
        y: y,
        z: z,
    })
}

/// Create a meshgrid for the lambert function
fn create_meshgrid(
    x_plane: &Array<f64, Dim<[usize; 1]>>,
    y_plane: &Array<f64, Dim<[usize; 1]>>,
) -> Result<(Array<f64, Dim<[usize; 2]>>, Array<f64, Dim<[usize; 2]>>), Box<dyn std::error::Error>>
{
    let mut new_x: Array<f64, Dim<[usize; 2]>> = Array::zeros([x_plane.len(), y_plane.len()]);
    let mut new_y: Array<f64, Dim<[usize; 2]>> = Array::zeros([x_plane.len(), y_plane.len()]);
    let mut counter = 0;
    let max_count = x_plane.len();
    for value in new_x.iter_mut() {
        *value = x_plane[counter];
        counter += 1;
        if counter >= max_count {
            counter = 0;
        }
    }

    counter = 0;
    let mut counter_y = 0;
    for value in new_y.iter_mut() {
        *value = y_plane[counter];
        counter_y += 1;
        if counter_y >= max_count {
            counter_y = 0;
            counter += 1;
        }
    }

    Ok((new_x, new_y))
}
