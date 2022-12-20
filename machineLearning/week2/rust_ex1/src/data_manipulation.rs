use ndarray::{Array2, s, Array1, stack, Axis, ArrayBase, OwnedRepr, Dimension, RemoveAxis};

pub fn feature_normalize<T: Dimension + RemoveAxis>(arr: ArrayBase<OwnedRepr<f64>, T>){
    let mut min_max = Vec::new();

    for col in arr.axis_iter(Axis(1)){
        
        let mut last_col_i = 0;
        let mut first_val = true;
        //println!("{}", col[0]);

        println!("{:?}", col);
        for i in col.iter(){
            if first_val {
                min_max.push((*i, *i));
                last_col_i = min_max.len()-1;
                first_val = false;
            }

            if i < &min_max[last_col_i].0 {
                min_max[last_col_i].0 = *i;
            }

            if i > &min_max[last_col_i].1 {
                min_max[last_col_i].1 = *i;
            }
        }
    }

    println!("{:?}", min_max);
}