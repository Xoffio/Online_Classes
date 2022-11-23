use plotters::prelude::*;
use ndarray::{stack, Axis, Array1};

pub fn plot_data(x: Array1::<f64>, y: Array1::<f64>){
    let data = stack!(Axis(1), x, y);
  
    // Create the bitmap ( where we are going to draw our data)
    let root_area = BitMapBackend::new("test1.png", (960, 540)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        // size of data label to the left, right, top, and bottom
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .build_cartesian_2d(0.0..600.0, 0.0..600.0) // Cartesian range and type
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