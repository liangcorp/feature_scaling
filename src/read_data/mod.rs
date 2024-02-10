//! # Read data from file and store the value into vectors
//!
use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;

type DoubleVecF64 = Vec<Vec<f64>>;

pub fn get_data(path: &Path) -> Result<(Box<DoubleVecF64>, Box<Vec<f64>>), io::Error> {
    let lines = match File::open(path) {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            return Err(Error::new(ErrorKind::NotFound, "File not found"));
        }
        Err(error) if error.kind() == ErrorKind::PermissionDenied => {
            return Err(Error::new(ErrorKind::PermissionDenied, "Permission denied"));
        }
        Err(_) => {
            return Err(Error::new(ErrorKind::Other, "Can not open file."));
        }
    };

    let mut x_slice: Vec<String> = Vec::new();
    let mut y: Vec<f64> = Vec::new();

    // Read the file line by line
    // split each line by the last ',' into two vectors of v and y
    for line in lines {
        if let Some(data_tuple) = line.unwrap().rsplit_once(',') {
            x_slice.push(data_tuple.0.to_string());
            y.push(data_tuple.1.parse::<f64>().expect("Failed"));
        }
    }

    let mut x: Vec<Vec<f64>> = Vec::new();
    for i in x_slice.iter() {
        x.push(i.split(',').map(|x| x.parse::<f64>().unwrap()).collect());
    }

    Ok((Box::new(x), Box::new(y)))
}
