use ndarray::{Array2, s, Array1, stack, Axis, arr1};
//use std::time::Instant;

pub fn compute_cost(xy: &Array2<f64>, theta: &Array1<f64>) -> f64 {
    //let now = Instant::now();
    let x_num_of_cols = xy.dim().0;
    let x = stack!(
        Axis(1), 
        Array1::<f64>::ones(x_num_of_cols), 
        xy.slice(s![.., 0]));
    let y = xy.slice(s![.., 1]);

    let cost = ((x.dot(theta)-y).mapv(|i| i.powf(2.0))).sum() / (2.0*(x_num_of_cols as f64));

    //let elapsed = now.elapsed().as_micros();
    //println!("Elapsed (μs): {}", elapsed);
    //println!("{:?}", cost  );

    // Return the cost
    cost
}

pub fn compute_cost_multi(x: &Array2<f64>, y: &Array1<f64>, theta: &Array1<f64>) -> f64 {
    //let now = Instant::now();
    let m = x.dim().0;

    let cost = ((x.dot(&theta.slice(s![1..]))-y).mapv(|i| (i+theta[0]).powf(2.0))).sum() / (2.0*(m as f64));
    
    //let elapsed = now.elapsed().as_micros();
    //println!("Elapsed (μs): {}", elapsed);
    println!("cost : {:?}", &cost);

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

pub fn gradient_descent_multi(x: &Array2<f64>, y: &Array1<f64>, theta: &mut Array1<f64>, alpha: f64, iterations: usize) -> Array1<f64>{
    let (m, _n) = x.dim();
    let mut j_history: Vec<f64> = Vec::new();

    for _i in 0..iterations{
        let hypo = x.dot(&theta.slice(s![1..])).mapv(|i| (i+theta[0])) - y;

        // In order to make the calculation faster
        // Instead of adding a column of ones to X
        // I will just do the dot product without the first extra column
        let pre_des_1 = hypo.dot(x);

        // Then make the first column with the result (Which is the sum of "hypo")
        let mut pre_des: Array1<f64> = arr1(&[hypo.sum()]);

        // Then finally append them together.
        // this array should have a length equal to the theta length
        pre_des.append(Axis(0), (&pre_des_1).into()).unwrap();

        let des = alpha * (1.0/m as f64) * pre_des;

        *theta = theta.clone() - des;
        //println!("theta: {}", theta);

       j_history.push(compute_cost_multi(x, y, theta)); 
    }

    println!("{}", theta);

    Array1::from_shape_vec(iterations, j_history).unwrap()

}