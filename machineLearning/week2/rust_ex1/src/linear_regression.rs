use ndarray::{Array2, s, Array1, stack, Axis};

pub fn compute_cost(xy: &Array2<f64>, theta: &Array1<f64>) -> f64 {
    let x_num_of_cols = xy.dim().0;
    let x = stack!(
        Axis(1), 
        Array1::<f64>::ones(x_num_of_cols), 
        xy.slice(s![.., 0]));
    let y = xy.slice(s![.., 1]);

    let cost = ((x.dot(theta)-y).mapv(|i| i.powf(2.0))).sum() / (2.0*(x_num_of_cols as f64));
    println!("{:?}", cost  );

    cost
}