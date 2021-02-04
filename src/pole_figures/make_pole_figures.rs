use ndarray::Array;
use ndarray::Zip;
use plotters::prelude::*;

use crate::configuration::particle_record::ParticleRecord;
use crate::pole_figures::{
    color_gradients::set_color_gradient, crystal_axis::CrystalAxes, lambert::Lambert,
    minerals::Mineral, percentage::Percentage, pole_figure::PoleFigure,
};

use std::path::Path;
use std::time::Instant;

/// The main function responsible for actually producing the the pole figures.
pub fn make_pole_figures(
    small_figure: bool,
    no_description_text: bool,
    elastisity_header: bool,
    n_grains: usize,
    _time_step: &u64,
    particle_id: u64,
    pole_figure_grid: &Vec<Vec<PoleFigure>>,
    lambert: &Lambert,
    output_file: &Path,
    particle_record: &ParticleRecord,
    time: f64,
    gam: f64,
    color_gradient_selection: String,
    max_count_method: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let before = Instant::now();

    let color_gradient = set_color_gradient(color_gradient_selection);

    // Grid of points is a square and it extends outside the pole figure circumference.
    // Create mask to only plot color and contours within the pole figure
    let mut mask = lambert.x_plane.clone();

    let mut circle_path: Vec<(f64, f64)> = Vec::new();
    Zip::from(&mut mask)
        .and(&lambert.x_plane)
        .and(&lambert.z_plane)
        .par_apply(|a, x, z| {
            let radius = (x * x + z * z).sqrt();
            if radius >= lambert.r_plane + 0.001 {
                *a = std::f64::NAN
            } else {
                *a = 1.
            }
        });

    // Create a boundary circle for the Schmidt Net
    let bd_theta = Array::linspace(0., 2. * std::f64::consts::PI, 100);
    let bd_center = [0.0, 0.0];
    let bd_radius = 2.0 / 2.0_f64.sqrt();

    for i in 0..bd_theta.len() {
        circle_path.push((
            bd_theta[i].sin() * bd_radius + bd_center[0],
            bd_theta[i].cos() * bd_radius + bd_center[1],
        ));
    }

    let figure_height = if small_figure { 500 } else { 800 };
    let legend_width = if small_figure { 150 } else { 200 };
    let number_of_figures_horizontal: usize = pole_figure_grid.len();
    if number_of_figures_horizontal < 1 {
        println!("No figures to make. Exit.");
        return Ok(());
    }

    let number_of_figures_vertical: usize = pole_figure_grid[0].len();
    if number_of_figures_vertical < 1 {
        println!("No figures to make. Exit.");
        return Ok(());
    }

    let total_figure_width: u32 = number_of_figures_horizontal as u32 * figure_height + 10;
    let total_figure_height: u32 = if elastisity_header {
        number_of_figures_vertical as u32 * figure_height + 100
    } else {
        number_of_figures_vertical as u32 * figure_height
    };

    println!("    Before drawing: Elapsed time: {:.2?}", before.elapsed());
    let path_string = output_file.to_string_lossy().into_owned();
    println!("    save file to {}", path_string);
    let root = BitMapBackend::new(
        &path_string,
        (total_figure_width + legend_width, total_figure_height),
    )
    .into_drawing_area();
    root.fill(&WHITE)?;

    println!("    made root: Elapsed time: {:.2?}", before.elapsed());
    let (header, body) = if elastisity_header {
        root.split_vertically(150)
    } else {
        root.split_vertically(0)
    };

    let hp = Percentage {
        total: figure_height as f64,
    };
    let wp = Percentage {
        total: total_figure_width as f64 / number_of_figures_horizontal as f64,
    };
    let font_size_header = if small_figure { 24 } else { 38 };
    let line_distance = 5.5;
    let top_margin = 0.25;
    let left_margin = 0.5;
    let font_type = "helvetica";

    if elastisity_header {
        // Do stuff in header
        println!(
            "    start computing anisotropy: Elapsed time: {:.2?}",
            before.elapsed()
        );
        // preprocessing particle data:
        let pr = particle_record;
        let full_norm_square = particle_record.full_norm_square.unwrap();
        let isotropic = pr.isotropic_norm_square.unwrap();

        let tric_unsorted = [
            pr.triclinic_norm_square_p1.unwrap(),
            pr.triclinic_norm_square_p2.unwrap(),
            pr.triclinic_norm_square_p3.unwrap(),
        ];
        let mono_unsorted = [
            pr.monoclinic_norm_square_p1.unwrap(),
            pr.monoclinic_norm_square_p2.unwrap(),
            pr.monoclinic_norm_square_p3.unwrap(),
        ];
        let orth_unsorted = [
            pr.orthohombic_norm_square_p1.unwrap(),
            pr.orthohombic_norm_square_p2.unwrap(),
            pr.orthohombic_norm_square_p3.unwrap(),
        ];
        let tetr_unsorted = [
            pr.tetragonal_norm_square_p1.unwrap(),
            pr.tetragonal_norm_square_p2.unwrap(),
            pr.tetragonal_norm_square_p3.unwrap(),
        ];
        let hexa_unsorted = [
            pr.hexagonal_norm_square_p1.unwrap(),
            pr.hexagonal_norm_square_p2.unwrap(),
            pr.hexagonal_norm_square_p3.unwrap(),
        ];

        let mut tric_sorted = tric_unsorted.clone();
        let mut mono_sorted = mono_unsorted.clone();
        let mut orth_sorted = orth_unsorted.clone();
        let mut tetr_sorted = tetr_unsorted.clone();
        let mut hexa_sorted = hexa_unsorted.clone();

        let total_anisotropy =
            tric_sorted[0] + mono_sorted[0] + orth_sorted[0] + tetr_sorted[0] + hexa_sorted[0];

        tric_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        mono_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        orth_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        tetr_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        hexa_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let tric_perc_full = tric_unsorted
            .iter()
            .map(|v| (v / full_norm_square) * 100.)
            .collect::<Vec<f64>>();
        let mono_perc_full = mono_unsorted
            .iter()
            .map(|v| (v / full_norm_square) * 100.)
            .collect::<Vec<f64>>();
        let orth_perc_full = orth_unsorted
            .iter()
            .map(|v| (v / full_norm_square) * 100.)
            .collect::<Vec<f64>>();
        let tetr_perc_full = tetr_unsorted
            .iter()
            .map(|v| (v / full_norm_square) * 100.)
            .collect::<Vec<f64>>();
        let hexa_perc_full = hexa_unsorted
            .iter()
            .map(|v| (v / full_norm_square) * 100.)
            .collect::<Vec<f64>>();

        println!(
            "    end computing anisotropy: Elapsed time: {:.2?}",
            before.elapsed()
        );
        println!("    start header: Elapsed time: {:.2?}", before.elapsed());

        header
        .draw(&Text::new(
            format!("id={},time={:.5e}, position=({:.3e}:{:.3e}:{:.3e}), ODT={:.4}, grains={}, anisotropic%={:.4}",
            particle_id,
            time,
            particle_record.x,
            particle_record.y,
            particle_record.z.unwrap(),
            particle_record.olivine_deformation_type.unwrap(),
            n_grains,((total_anisotropy)/full_norm_square)*100.),
            ((wp.calc(left_margin) ) as i32, hp.calc(top_margin) as i32),
            (font_type, font_size_header).into_font(),
        ))?;

        header.draw(&Text::new(
            format!(
                "hex%={:.2},{:.2},{:.2}",
                hexa_perc_full[0], hexa_perc_full[1], hexa_perc_full[2]
            ),
            (
                (left_margin) as i32,
                hp.calc(top_margin + 1.0 * line_distance) as i32,
            ),
            (font_type, font_size_header).into_font(),
        ))?;

        header.draw(&Text::new(
            format!(
                "h/a%={:.2},{:.2},{:.2}",
                ((hexa_unsorted[0]) / (total_anisotropy)) * 100.,
                ((hexa_unsorted[1]) / (total_anisotropy)) * 100.,
                ((hexa_unsorted[2]) / (total_anisotropy)) * 100.
            ),
            (
                (left_margin) as i32,
                hp.calc(top_margin + 2.0 * line_distance) as i32,
            ),
            (font_type, font_size_header).into_font(),
        ))?;

        println!("    mid header 3: Elapsed time: {:.2?}", before.elapsed());
        header.draw(&Text::new(
            format!(
                "tet%={:.2},{:.2},{:.2}",
                tetr_perc_full[0], tetr_perc_full[1], tetr_perc_full[2]
            ),
            (
                (left_margin + (total_figure_width as f64 + legend_width as f64 + 10.0) * 0.2)
                    as i32,
                hp.calc(top_margin + 1.0 * line_distance) as i32,
            ),
            (font_type, font_size_header).into_font(),
        ))?;
        println!("    mid header 4: Elapsed time: {:.2?}", before.elapsed());
        header.draw(&Text::new(
            format!(
                "t/a%={:.2},{:.2},{:.2}",
                ((tetr_unsorted[0]) / (total_anisotropy)) * 100.,
                ((tetr_unsorted[1]) / (total_anisotropy)) * 100.,
                ((tetr_unsorted[2]) / (total_anisotropy)) * 100.
            ),
            (
                (left_margin + (total_figure_width as f64 + legend_width as f64 + 10.0) * 0.2)
                    as i32,
                hp.calc(top_margin + 2.0 * line_distance) as i32,
            ),
            (font_type, font_size_header).into_font(),
        ))?;
        header.draw(&Text::new(
            format!(
                "ort%={:.2},{:.2},{:.2}",
                orth_perc_full[0], orth_perc_full[1], orth_perc_full[2]
            ),
            (
                (left_margin + (total_figure_width as f64 + legend_width as f64 + 10.0) * 0.4)
                    as i32,
                hp.calc(top_margin + 1.0 * line_distance) as i32,
            ),
            (font_type, font_size_header).into_font(),
        ))?;
        header.draw(&Text::new(
            format!(
                "o/a%={:.2},{:.2},{:.2}",
                ((orth_unsorted[0]) / (total_anisotropy)) * 100.,
                ((orth_unsorted[1]) / (full_norm_square - isotropic)) * 100.,
                ((orth_unsorted[2]) / (full_norm_square - isotropic)) * 100.
            ),
            (
                (left_margin + (total_figure_width as f64 + legend_width as f64 + 10.0) * 0.4)
                    as i32,
                hp.calc(top_margin + 2.0 * line_distance) as i32,
            ),
            (font_type, font_size_header).into_font(),
        ))?;
        header.draw(&Text::new(
            format!(
                "mon%={:.2},{:.2},{:.2}",
                mono_perc_full[0], mono_perc_full[1], mono_perc_full[2]
            ),
            (
                (left_margin + (total_figure_width as f64 + legend_width as f64 + 10.0) * 0.6)
                    as i32,
                hp.calc(top_margin + 1.0 * line_distance) as i32,
            ),
            (font_type, font_size_header).into_font(),
        ))?;
        header.draw(&Text::new(
            format!(
                "m/a%={:.2},{:.2},{:.2}",
                ((mono_unsorted[0]) / (total_anisotropy)) * 100.,
                ((mono_unsorted[1]) / (full_norm_square - isotropic)) * 100.,
                ((mono_unsorted[2]) / (full_norm_square - isotropic)) * 100.
            ),
            (
                (left_margin + (total_figure_width as f64 + legend_width as f64 + 10.0) * 0.6)
                    as i32,
                hp.calc(top_margin + 2.0 * line_distance) as i32,
            ),
            (font_type, font_size_header).into_font(),
        ))?;
        header.draw(&Text::new(
            format!(
                "tri%={:.2},{:.2},{:.2}",
                tric_perc_full[0], tric_perc_full[1], tric_perc_full[2]
            ),
            (
                (left_margin + (total_figure_width as f64 + legend_width as f64 + 10.0) * 0.8)
                    as i32,
                hp.calc(top_margin + 1.0 * line_distance) as i32,
            ),
            (font_type, font_size_header).into_font(),
        ))?;
        header.draw(&Text::new(
            format!(
                "t/a%={:.2},{:.2},{:.2}",
                ((tric_unsorted[0]) / (total_anisotropy)) * 100.,
                ((tric_unsorted[1]) / (full_norm_square - isotropic)) * 100.,
                ((tric_unsorted[2]) / (full_norm_square - isotropic)) * 100.
            ),
            (
                (left_margin + (total_figure_width as f64 + legend_width as f64 + 10.0) * 0.8)
                    as i32,
                hp.calc(top_margin + 2.0 * line_distance) as i32,
            ),
            (font_type, font_size_header).into_font(),
        ))?;

        println!("    end header: Elapsed time: {:.2?}", before.elapsed());
    }
    println!("    start body: Elapsed time: {:.2?}", before.elapsed());
    // do stuff in body:

    let font_size_figure = if small_figure { 35 } else { 45 };
    let (left, right) = body.split_horizontally(total_figure_width);

    let drawing_areas_horizontal = left.split_evenly((1, number_of_figures_horizontal));

    //println!("    number of figures = {}", number_of_figures);
    for horizontal_figure_number in 0..number_of_figures_horizontal {
        let drawing_areas_vertical = drawing_areas_horizontal[horizontal_figure_number]
            .split_evenly((number_of_figures_vertical, 1));
        let right_areas = right.split_evenly((number_of_figures_vertical, 1));

        for vertical_figure_number in 0..number_of_figures_vertical {
            println!(
                "horizontal_figure_number:vertical_figure_number = {}:{}",
                horizontal_figure_number, vertical_figure_number
            );

            let max_count_value_original =
                &pole_figure_grid[horizontal_figure_number][vertical_figure_number].max_count;

            let max_count_value_colorscale = match &max_count_method[..] {
                "divide 2" => max_count_value_original / 2.0,
                "divide 3" => max_count_value_original / 3.0,
                "divide 4" => max_count_value_original / 4.0,
                _ => *max_count_value_original,
            };

            if horizontal_figure_number == 0 {
                let mut chart = ChartBuilder::on(&right_areas[vertical_figure_number])
                    .margin(25)
                    .margin_right(2)
                    .margin_left(10)
                    .top_x_label_area_size(0)
                    .y_label_area_size(100)
                    .caption(
                        if small_figure {
                            format!(
                                "{:.2}{}",
                                *max_count_value_original,
                                match &max_count_method[..] {
                                    "divide 2" => "/2",
                                    "divide 3" => "/3",
                                    "divide 4" => "/4",
                                    _ => "",
                                }
                            )
                        } else {
                            format!(
                                "{:.2}{}",
                                *max_count_value_original,
                                match &max_count_method[..] {
                                    "divide 2" => "/2",
                                    "divide 3" => "/3",
                                    "divide 4" => "/4",
                                    _ => "",
                                }
                            )
                        },
                        ("helvetica", font_size_figure),
                    )
                    .build_cartesian_2d(0.0..1.0, 0.0..max_count_value_colorscale)?;

                chart
                    .configure_mesh()
                    .x_labels(0)
                    .y_labels(10)
                    .y_label_offset(15)
                    .disable_x_mesh()
                    .disable_y_mesh()
                    .label_style(("helvetica", font_size_figure))
                    .draw()?;

                let legend_size = 151;

                let mut matrix = [max_count_value_colorscale; 151];

                for i in 0..legend_size - 1 {
                    matrix[i] = i as f64 * max_count_value_colorscale / (legend_size as f64 - 1.0);
                }

                for i in 0..legend_size - 1 {
                    let picked_color = color_gradient.get(
                        ((matrix[i]).powf(gam) / (max_count_value_colorscale.powf(gam))) as f32,
                    );
                    let picked_rgb_color = RGBColor(
                        (picked_color.red * 255.0) as u8,
                        (picked_color.green * 255.0) as u8,
                        (picked_color.blue * 255.0) as u8,
                    );
                    chart.draw_series(std::iter::once(Rectangle::new(
                        [(0.0, matrix[i]), (0.0 + 1.0, matrix[i + 1])],
                        picked_rgb_color.filled(),
                    )))?;
                }
            }

            let crystal_axis_string = match pole_figure_grid[horizontal_figure_number]
                [vertical_figure_number]
                .crystal_axis
            {
                CrystalAxes::AAxis => "a-axis",
                CrystalAxes::BAxis => "b-axis",
                CrystalAxes::CAxis => "c-axis",
            };
            let mineral_string =
                match pole_figure_grid[horizontal_figure_number][vertical_figure_number].mineral {
                    Mineral::Olivine => "olivine",
                    Mineral::Enstatite => "enstatite",
                };
            let mut chart = ChartBuilder::on(&drawing_areas_vertical[vertical_figure_number])
                .build_cartesian_2d(
                    -lambert.r_plane - 0.05..lambert.r_plane + 0.15,
                    -lambert.r_plane - 0.05..lambert.r_plane + 0.15,
                )?;
            let counts = &pole_figure_grid[horizontal_figure_number][vertical_figure_number].counts;
            let npts = counts.shape()[0];

            let mut current: Vec<Vec<Vec<(f64, f64)>>> = Vec::new();
            for i in 0..npts - 1 {
                let mut current_i: Vec<Vec<(f64, f64)>> = Vec::new();
                for j in 0..npts - 1 {
                    current_i.push(vec![
                        (lambert.x_plane[[i + 1, j]], lambert.z_plane[[i + 1, j]]),
                        (
                            lambert.x_plane[[i + 1, j + 1]],
                            lambert.z_plane[[i + 1, j + 1]],
                        ),
                        (lambert.x_plane[[i, j + 1]], lambert.z_plane[[i, j + 1]]),
                        (lambert.x_plane[[i, j]], lambert.z_plane[[i, j]]),
                    ]);
                }
                current.push(current_i);
            }

            let mut total_mask = mask.clone();
            for i in 0..npts - 1 {
                for j in 0..npts - 1 {
                    if !mask[[i, j]].is_nan()
                        && !mask[[i, j + 1]].is_nan()
                        && !mask[[i + 1, j + 1]].is_nan()
                        && !mask[[i + 1, j]].is_nan()
                    {
                        total_mask[[i, j]] = 1.0;
                    } else {
                        total_mask[[i, j]] = std::f64::NAN;
                    }
                }
            }
            println!(
                "      before 1st drawing: Elapsed time: {:.2?}",
                before.elapsed()
            );

            for i in 0..npts - 1 {
                for j in 0..npts - 1 {
                    let picked_color = color_gradient.get(
                        ((counts[[i, j]]).powf(gam) / (max_count_value_colorscale.powf(gam)))
                            as f32,
                    );
                    let picked_rgb_color = RGBColor(
                        (picked_color.red * 255.0) as u8,
                        (picked_color.green * 255.0) as u8,
                        (picked_color.blue * 255.0) as u8,
                    );

                    if !mask[[i, j]].is_nan() {
                        chart.draw_series(std::iter::once(Polygon::new(
                            current[i][j].clone(),
                            picked_rgb_color.filled(),
                        )))?;
                    }
                }
            }
            chart.draw_series(std::iter::once(PathElement::new(
                circle_path.clone(),
                Into::<ShapeStyle>::into(&BLACK).stroke_width(5),
            )))?;

            println!(
                "      before 2st drawing: Elapsed time: {:.2?}",
                before.elapsed()
            );
            if !no_description_text {
                drawing_areas_vertical[vertical_figure_number]
                    .draw(&Text::new(
                        format!("{}", crystal_axis_string),
                        (
                            wp.calc(left_margin) as i32,
                            hp.calc(top_margin + 1.0 * line_distance) as i32,
                        ),
                        (font_type, font_size_figure, FontStyle::Bold).into_font(),
                    ))
                    .unwrap();
                drawing_areas_vertical[vertical_figure_number].draw(&Text::new(
                    format!("{}", mineral_string),
                    (
                        wp.calc(left_margin) as i32,
                        hp.calc(top_margin + 0.0 * line_distance) as i32,
                    ),
                    (font_type, font_size_figure, FontStyle::Bold).into_font(),
                ))?;
            }
            drawing_areas_vertical[vertical_figure_number].draw(&Text::new(
                format!("Z"),
                (wp.calc(46.4) as i32, (hp.calc(11.) - 85.) as i32),
                (font_type, font_size_figure).into_font(),
            ))?;
            drawing_areas_vertical[vertical_figure_number].draw(&Text::new(
                format!("X"),
                (
                    wp.calc(96.0) as i32,
                    if small_figure { 250 } else { 400 } as i32,
                ),
                (font_type, font_size_figure).into_font(),
            ))?;
            // end of for loop
        }

        println!(
            "      made one of the figures: Elapsed time: {:.2?}",
            before.elapsed()
        );
    }

    println!(
        "    After make_polefigures: Elapsed time: {:.2?}",
        before.elapsed()
    );

    Ok(())
}
