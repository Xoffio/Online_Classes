use ndarray::{Array2};

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