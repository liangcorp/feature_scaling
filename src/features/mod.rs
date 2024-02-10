//! # Use mean normalization on 2D array.
//! This is used on X that usually contains multiple features.
//!
#[allow(dead_code)]
type DoubleVecF64 = Vec<Vec<f64>>;

pub fn mean_normal(x: &[Vec<f64>]) -> Box<DoubleVecF64> {
    let mut max: Vec<f64> = Vec::new();
    let mut min: Vec<f64> = Vec::new();
    let mut mean: Vec<f64> = Vec::new();
    let mut std_dev: Vec<f64> = Vec::new();

    let mut result: DoubleVecF64 = x.to_vec();

    let row = x.len();
    let col = x[0].len();

    let mut sum: f64;

    // Set max and min for each feature
    for i in x[0].iter() {
        max.push(*i);
        min.push(*i);
    }

    // Find max and min for each feature
    // Each column is a feature, this means
    //  we need to loop from column to row.
    for j in 0..col {
        sum = 0.0;
        for i in x.iter().enumerate().take(row) {
            if max[j] < x[i.0][j] {
                max[j] = x[i.0][j];
            } else if min[j] > x[i.0][j] {
                min[j] = x[i.0][j];
            } else {
                // Do nothing
            }
            sum += x[i.0][j];
        }
        mean.push(sum);
    }

    // find mean for each feature
    for j in mean.iter_mut().take(col) {
        *j /= row as f64;
    }


    //  Loop from colum to row.
    //  Calculate the standard deviation for each feature
    for j in 0..col {
        sum = 0.0;
        for i in x.iter().take(row) {
            sum += (i[j] - mean[j]) * (i[j] - mean[j]);
        }

        std_dev.push((sum / x.len() as f64).sqrt());
    }

    //  Set the value of new 2D arry to normalized value
    for j in 0..col {
        for i in 0..row {
            result[i][j] = (x[i][j] - mean[j]) / std_dev[j];
        }
    }

    Box::new(result)
}
