//! # Use mean normalization on 2D array.
//! This is used on X that usually contains multiple features.
//! The function returns a pointer to a structure.
//! The structure contains pointers to the following:
//!     - Pointer to the result 2D array
//!     - Pointer to the list of mean
//!     - Pointer to the list of standard deviation
//!
//! NOTE: Run free on pointers of the structure
//! and its elements in main.
//!
#[allow(dead_code)]
type DoubleVecF64 = Vec<Vec<f64>>;

pub fn mean_normal(v: &[Vec<f64>]) -> (Box<DoubleVecF64>, Box<Vec<f64>>, Box<Vec<f64>>) {
    let mut max: Vec<f64> = Vec::new();
    let mut min: Vec<f64> = Vec::new();
    let mut mean: Vec<f64> = Vec::new();
    let mut std_dev: Vec<f64> = Vec::new();

    let mut result: DoubleVecF64 = v.to_vec();

    let row = v.len();
    let col = v[0].len();

    let mut sum: f64;

    for i in v[0].iter() {
        max.push(*i);
        min.push(*i);
    }

    mean.push(1.0);

    for j in 1..col {
        sum = 0.0;
        for i in v.iter().enumerate().take(row) {
            if max[j] < v[i.0][j] {
                max[j] = v[i.0][j];
            } else if min[j] > v[i.0][j] {
                min[j] = v[i.0][j];
            } else {
                // Do nothing
            }
            sum += v[i.0][j];
        }
        mean.push(sum);
    }

    for j in mean.iter_mut().take(col).skip(1) {
        *j /= row as f64;
    }

    std_dev.push(1.0);

    for j in 1..col {
        sum = 0.0;
        for i in v.iter().take(row) {
            sum += (i[j] - mean[j]) * (i[j] - mean[j]);
        }

        std_dev.push((sum / v.len() as f64).sqrt());
    }

    for j in 1..col {
        for i in 0..row {
            result[i][j] = (v[i][j] - mean[j]) / std_dev[j];
        }
    }

    (Box::new(result), Box::new(mean), Box::new(std_dev))
}
