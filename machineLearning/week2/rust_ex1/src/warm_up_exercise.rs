use ndarray::{Array2};

pub fn warm_up_exercise() -> Array2<i32>{
    let a = Array2::<i32>::eye(5);
    println!("{}", a);
    a
}