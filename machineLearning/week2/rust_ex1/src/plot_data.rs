use plotters::prelude::*;
use ndarray::{stack, Axis, Array1, Array2};

pub fn plot_data_arr1(x: Array1::<f64>, y: Array1::<f64>, out_path: &str){
    let data = stack!(Axis(1), x, y);
  
    // Create the bitmap ( where we are going to draw our data)
    let root_area = BitMapBackend::new(out_path, (960, 540)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        // size of data label to the left, right, top, and bottom
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .build_cartesian_2d(0.0..25.0, 0.0..25.0) // Cartesian range and type
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    for i in data.axis_iter(Axis(0)).map(|p| (p[0], p[1])){
        println!("{:?}", i);
    }

    // Draw Scatter Plot
    ctx.draw_series(
        data.axis_iter(Axis(0)).map(|p| Circle::new((p[0], p[1]), 4.0_f64, &BLUE)),
    ).unwrap();
}

pub fn plot_data_arr2(xy: &Array2::<f64>, out_path: &str){
  
    // Create the bitmap ( where we are going to draw our data)
    let root_area = BitMapBackend::new(out_path, (960, 540)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        // size of data label to the left, right, top, and bottom
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .build_cartesian_2d(0.0..25.0, 0.0..25.0) // Cartesian range and type
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    // Draw Scatter Plot
    ctx.draw_series(
        xy.axis_iter(Axis(0)).map(|p| Circle::new((p[0], p[1]), 3.0_f64, &RED)),
    ).unwrap();
}

pub fn plot_data_arr2_and_pred_line(xy: &Array2::<f64>, theta: &Array1::<f64>, out_path: &str){
  
    // Create the bitmap ( where we are going to draw our data)
    let root_area = BitMapBackend::new(out_path, (960, 540)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        // size of data label to the left, right, top, and bottom
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .build_cartesian_2d(0.0..25.0, 0.0..25.0) // Cartesian range and type
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    // Draw Scatter Plot
    ctx.draw_series(
        xy.axis_iter(Axis(0)).map(|p| Circle::new((p[0], p[1]), 3.0_f64, &RED)),
    ).unwrap();

    ctx.draw_series(
        LineSeries::new((0..=25).map(|x| (x as f64, theta[0] + (x as f64) * theta[1])), &BLUE),
    ).unwrap();
}

pub fn plot_data_debug_j(xy: &Array1::<f64>, out_path: &str){
    let arr_len = xy.len();
    let mut min = xy[0];
    let mut max = xy[0];

    for y in xy.iter() {
        if y < &min {
            min = *y;
        }

        if y > &max {
            max = *y;
        }
    }
  
    // Create the bitmap ( where we are going to draw our data)
    let root_area = BitMapBackend::new(out_path, (960, 540)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        // size of data label to the left, right, top, and bottom
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .build_cartesian_2d(0.0..(arr_len as f64), (min as f64)..(max as f64)) // Cartesian range and type
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        LineSeries::new((0..=(arr_len-1)).map(|x| (x as f64, xy[x])), &BLUE),
    ).unwrap();
}