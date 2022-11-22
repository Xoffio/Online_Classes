use ndarray::{Array2};
use std::io::{stdin, Read};

mod warm_up_exercise;


fn main() {
    println!("Running warmUpExercise ... \n");
    println!("5x5 Identity Matrix: \n");

    assert_eq!(warm_up_exercise::warm_up_exercise(), Array2::<i32>::eye(5));

    println!("Program paused. Press enter to continue.\n");
    stdin().read(&mut [0]).unwrap();
    

}
