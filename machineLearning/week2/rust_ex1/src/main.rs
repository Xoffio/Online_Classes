use ndarray::{Array2, arr1, Array1, s};
use std::io::{stdin, Read};

mod warm_up_exercise;
mod plot_data;
mod reader;
mod linear_regression;
mod data_manipulation;


fn main() {
    // ==================== Part 1: Basic Function ====================
    println!("Running warmUpExercise ... \n");
    println!("5x5 Identity Matrix: \n");

    assert_eq!(warm_up_exercise::warm_up_exercise(), Array2::<i32>::eye(5));

    print!("Program paused. Press enter to continue.\n");
    stdin().read(&mut [0]).unwrap();
    
    // ======================= Part 2: Plotting =======================

    println!("Plotting Data ...");

    let vec01 = arr1(&[10.0, 20.0, 3.0]);
    let vec02 = arr1(&[4.0, 5.0, 6.]);
    plot_data::plot_data_arr1(vec01, vec02, "test1.png");

    let data = reader::read_to_arr2("../machine-learning-ex1/ex1/ex1data1.txt");
    plot_data::plot_data_arr2(&data, "test2.png");

    print!("Program paused. Press enter to continue.\n");
    stdin().read(&mut [0]).unwrap();

    // =================== Part 3: Cost and Gradient descent ===================

    let mut theta= Array1::<f64>::zeros(2);

    // Some gradient descent settings
    let iterations = 1500;
    let alpha = 0.01;

    println!("\nTesting the cost function ...");

    // compute and display initial cost
    let mut j = linear_regression::compute_cost(&data, &theta);
    println!("With theta = [0 ; 0]\nCost computed = {:.2}", j);
    println!("Expected cost value (approx) 32.07");

    // further testing of the cost function
    theta = arr1(&[-1.0, 2.0]);
    j = linear_regression::compute_cost(&data, &theta);
    println!("\nWith theta = [-1 ; 2]\nCost computed = {:.2}", j);
    println!("Expected cost value (approx) 54.24");

    print!("Program paused. Press enter to continue.\n");
    stdin().read(&mut [0]).unwrap();

    println!("\nRunning Gradient Descent ...");
    // run gradient descent
    theta = arr1(&[0.0, 0.0]);
    linear_regression::gradient_descent(&data, &mut theta, alpha, iterations);

    // print theta to screen
    println!("Theta found by gradient descent: {:.4}", theta);
    println!("Expected theta values (approx) [-3.6303,  1.1664]\n");

    // Plot the linear fit
    plot_data::plot_data_arr2_and_pred_line(&data, &theta, "test3.png");

    // Predict values for population sizes of 35,000 and 70,000
    let predict1 =  arr1(&[1.0, 3.5]).dot(&theta);
    println!("For population = 35,000, we predict a profit of {:.2}", predict1*10000.0);
    
    let predict2 = arr1(&[1.0, 7.0]).dot(&theta);
    println!("For population = 70,000, we predict a profit of {:.2}\n", predict2*10000.0);

    println!("Program paused. Press enter to continue.\n");
    stdin().read(&mut [0]).unwrap();

    // ================ MULTI ================
    // ================ Part 1: Feature Normalization ================

    println!("Loading data ...\n");

    // Load Data
    let (mut x, y) = reader::read_xs_y("../machine-learning-ex1/ex1/ex1data2.txt", 0..2, 2);
    //m = length(y);

    // Print out some data points
    println!("First 10 examples from the dataset:");
    println!(" x = {},\n y = {} \n", x.slice(s![0..10, ..]), y.slice(s![0..10]));

    //let theta = arr1(&[0.5, 0.5, 0.5]);
    let theta = arr1(&[0.0, 0.0, 0.0]);
    linear_regression::compute_cost_multi(&x, &y, &theta);

    println!("Program paused. Press enter to continue.\n");
    stdin().read(&mut [0]).unwrap();

    // Scale features and set them to zero mean
    println!("Normalizing Features ...");
    data_manipulation::feature_normalize(&mut x);
    println!("{:?}", x);



    let mut theta = arr1(&[0.0, 2.0, 3.0]);
    linear_regression::gradient_descent_multi(&x, &y, &mut theta, alpha, iterations);

    //[X mu sigma] = featureNormalize(X);

    // Add intercept term to X
    //X = [ones(m, 1) X];


    // ================ Part 2: Gradient Descent ================
}
