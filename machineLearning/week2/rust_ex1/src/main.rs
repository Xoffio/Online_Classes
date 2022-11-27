use ndarray::{Array2, arr1};
use std::io::{stdin, Read};

mod warm_up_exercise;
mod plot_data;
mod reader;
mod linear_regression;


fn main() {
    // ==================== Part 1: Basic Function ====================
    println!("Running warmUpExercise ... \n");
    println!("5x5 Identity Matrix: \n");

    assert_eq!(warm_up_exercise::warm_up_exercise(), Array2::<i32>::eye(5));

    print!("Program paused. Press enter to continue.\n");
    stdin().read(&mut [0]).unwrap();
    
    // ======================= Part 2: Plotting =======================

    let vec01 = arr1(&[10.0, 20.0, 3.0]);
    let vec02 = arr1(&[4.0, 5.0, 6.]);
    plot_data::plot_data_arr1(vec01, vec02, "test1.png");

    let data = reader::read_to_arr2("../machine-learning-ex1/ex1/ex1data1.txt");
    plot_data::plot_data_arr2(&data, "test2.png");

    print!("Program paused. Press enter to continue.\n");
    stdin().read(&mut [0]).unwrap();

    // =================== Part 3: Cost and Gradient descent ===================

    let theta = arr1(&[-1.0, 2.0]);

    linear_regression::compute_cost(&data, &theta);
}
