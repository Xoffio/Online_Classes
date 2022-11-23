use ndarray::{Array2, arr1};
use std::io::{stdin, Read};

mod warm_up_exercise;
mod plot_data;


fn main() {
    // ==================== Part 1: Basic Function ====================
    println!("Running warmUpExercise ... \n");
    println!("5x5 Identity Matrix: \n");

    assert_eq!(warm_up_exercise::warm_up_exercise(), Array2::<i32>::eye(5));

    println!("Program paused. Press enter to continue.\n");
    stdin().read(&mut [0]).unwrap();
    
    // ======================= Part 2: Plotting =======================

    let vec01 = arr1(&[10.0, 20.0, 300.0]);
    let vec02 = arr1(&[40.0, 50.0, 600.]);
    plot_data::plot_data(vec01, vec02);
}
