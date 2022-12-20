use ndarray::{Array2, Array, Array1};

pub fn read_to_arr2(file_path: &str) -> Array2<f64>{

    // File Reader.
    let mut csv_file = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)
        .expect("Error reading the file.");

    // Iterate through all the lines/rows in the file.
    let r_matrix: Array2<f64> = csv_file.records().map(|line| {
        let entire_line = line.expect("Error reading line");
        [entire_line[0].parse::<f64>().unwrap(), entire_line[1].parse::<f64>().unwrap()]
    }).collect::<Vec<_>>().into();

    //println!("{:?}", r_matrix);
    r_matrix
}

pub fn read_xs_y(file_path: &str, x_range: std::ops::Range<usize>, yi: usize) -> (Array2<f64>, Array1<f64>) {
    // n = number of features/variables
    let n = x_range.len();
 
    // File Reader.
    let mut csv_file = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)
        .expect("Error reading the file.");

    let mut y_matrix = Vec::new();
    let mut x_matrix = Vec::new();

    // Iterate through all the lines/rows in the file.
    // and get the values into the vectors
    for record in csv_file.records() {
        let entire_line = record.expect("Error reading line");

        for xi in x_range.clone(){
            x_matrix.push(entire_line[xi].parse::<f64>().unwrap());
        }

        y_matrix.push(entire_line[yi].parse::<f64>().unwrap());
    }

    // Transform the vectors into ndarray matrices
    let y_matrix = Array::from_vec(y_matrix);
    let x_matrix = Array::from_shape_vec((x_matrix.len()/n, n), x_matrix).unwrap();

    //println!("{:?}", x_matrix);
    //println!("{:?}", y_matrix);
    (x_matrix, y_matrix)
}