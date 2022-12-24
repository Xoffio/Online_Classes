use ndarray::{Axis, ArrayBase, OwnedRepr, Dimension, RemoveAxis};

pub fn feature_normalize<T: Dimension + RemoveAxis>(arr: &mut ArrayBase<OwnedRepr<f64>, T>){
    let mut min_max = Vec::new();

    // Get the min and max for each column
    for col in arr.axis_iter(Axis(1)){
        
        let mut last_col_i = 0;
        let mut first_val = true;
        let n_features = col.len() as f64;
        //println!("{}", col[0]);

        println!("{:?}", col);
        for i in col.iter(){
            if first_val {
                min_max.push((*i, *i, 0.0, 0.0));
                last_col_i = min_max.len()-1;
                first_val = false;
            }

            min_max[last_col_i].2 += i;

            if i < &min_max[last_col_i].0 {
                min_max[last_col_i].0 = *i;
            }

            if i > &min_max[last_col_i].1 {
                min_max[last_col_i].1 = *i;
            }
        }

        min_max[last_col_i].2 = min_max[last_col_i].2/n_features;
        min_max[last_col_i].3 = col.std(1.0);
    }


    let mut min_max_iter = min_max.iter();
    // Lets normalize the matrix
    for mut col in arr.axis_iter_mut(Axis(1)){
        let (_min, _max, mean, stand_der) = min_max_iter.next().unwrap();

        //println!("{:?}", col);
        //println!("iter {:?}", min_max_iter.next());
        for i in col.iter_mut(){
            //*i = (*i-mean)/(max - min);
            *i = (*i-mean)/stand_der;
        }
        println!("{:.3?} {:.3?}", mean, stand_der);
    }

    println!("{:?}", min_max);
}