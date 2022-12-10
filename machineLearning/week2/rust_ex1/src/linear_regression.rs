use ndarray::{Array2, s, Array1, stack, Axis};

pub fn compute_cost(xy: &Array2<f64>, theta: &Array1<f64>) -> f64 {
    let x_num_of_cols = xy.dim().0;
    let x = stack!(
        Axis(1), 
        Array1::<f64>::ones(x_num_of_cols), 
        xy.slice(s![.., 0]));
    let y = xy.slice(s![.., 1]);

    let cost = ((x.dot(theta)-y).mapv(|i| i.powf(2.0))).sum() / (2.0*(x_num_of_cols as f64));
    //println!("{:?}", cost  );

    // Return the cost
    cost
}

pub fn gradient_descent(xy: &Array2<f64>, theta: &mut Array1<f64>, alpha: f64, iterations: usize){
    let x_num_of_cols = xy.dim().0;
    let x = stack!(
        Axis(1), 
        Array1::<f64>::ones(x_num_of_cols), 
        xy.slice(s![.., 0]));
    let y = xy.slice(s![.., 1]);

    //println!("{:?}", theta);

    for _i in 0..iterations{
        let pre_des: Array1<f64> = x.dot(theta)-y;
        let des = alpha * (1.0/x_num_of_cols as f64) * pre_des.dot(&x);

        *theta = theta.clone() - des;

        //compute_cost(xy, theta);
        //println!("{}", theta);
    }

}