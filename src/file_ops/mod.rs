//! # Read data from file and store the value into vectors
//!
use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind, Write};
use std::path::Path;

type DoubleVecF64 = Vec<Vec<f32>>;

/// Read data from text file into vectors
pub fn read_data(path: &Path) -> Result<(Box<DoubleVecF64>, Box<Vec<f32>>), io::Error> {
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

    let mut x_slice: Vec<String> = vec![];
    let mut y: Vec<f32> = vec![];

    // Read the file line by line
    // split each line by the last ',' into two vectors of v and y
    for line in lines {
        if let Some(data_tuple) = line.unwrap().rsplit_once(',') {
            x_slice.push(data_tuple.0.to_string());
            y.push(data_tuple.1.parse::<f32>().expect("Failed"));
        }
    }

    let mut x: Vec<Vec<f32>> = vec![];
    for i in x_slice.iter() {
        x.push(i.split(',').map(|x| x.parse::<f32>().unwrap()).collect());
    }

    Ok((Box::new(x), Box::new(y)))
}

/// Write data into text file
pub fn write_data(result_x: &[Vec<f32>], result_y: &[f32], path: &Path) -> Result<(), io::Error> {
    let mut file = match File::create(path) {
        Ok(f) => io::BufWriter::new(f),
        Err(error) if error.kind() == ErrorKind::PermissionDenied => {
            return Err(Error::new(ErrorKind::PermissionDenied, "Permission denied"));
        }
        Err(_) => {
            return Err(Error::new(ErrorKind::Other, "Can not open file."));
        }
    };

    for (i, x_set) in result_x.iter().enumerate() {
        for x_v in x_set.iter() {
            file.write_all(format!("{},", x_v).as_bytes())?;
        }
        file.write_all(format!("{}\n", result_y[i]).as_bytes())?;
    }

    Ok(())
}
