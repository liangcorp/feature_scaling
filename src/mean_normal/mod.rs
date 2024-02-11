//! # Use mean normalization on 2D and 1D array.
//!

type DoubleVecF64 = Vec<Vec<f64>>;

/// Mean normalization for X.
/// X represent features which contains more than one field
/// Therefore, it is represented as a 2D vector
pub fn features(x: &[Vec<f64>]) -> Box<DoubleVecF64> {
    let mut vec_max: Vec<f64> = Vec::new();
    let mut vec_min: Vec<f64> = Vec::new();
    let mut vec_main: Vec<f64> = Vec::new();
    let mut std_deviations: Vec<f64> = Vec::new();

    let mut result: DoubleVecF64 = x.to_vec();

    let row = x.len();
    let col = x[0].len();

    let mut vec_sum: f64;

    // Set max and min for each feature
    for i in x[0].iter() {
        vec_max.push(*i);
        vec_min.push(*i);
    }

    // Find max, min and sum of total of each feature
    // Each column is a feature, this means
    //  we need to loop from column to row.
    for j in 0..col {
        vec_sum = 0.0;
        for x_row in x.iter() {
            if vec_max[j] < x_row[j] {
                vec_max[j] = x_row[j];
            } else if vec_min[j] > x_row[j] {
                vec_min[j] = x_row[j];
            } else {
                // Do nothing
            }
            vec_sum += x_row[j];
        }
        vec_main.push(vec_sum);
    }

    // find mean for each feature
    for mean in vec_main.iter_mut() {
        *mean /= row as f64;
    }

    //  Loop from colum to row.
    //  Calculate the standard deviation for each feature
    for j in 0..col {
        vec_sum = 0.0;
        for i in x.iter() {
            vec_sum += (i[j] - vec_main[j]) * (i[j] - vec_main[j]);
        }

        std_deviations.push((vec_sum / x.len() as f64).sqrt());
    }

    //  Set the value of new 2D arry to normalized value
    for j in 0..col {
        for i in 0..row {
            result[i][j] = (x[i][j] - vec_main[j]) / std_deviations[j];
        }
    }

    Box::new(result)
}

/// Mean normalization for Y. Y only has one field.
/// It's represented as a 1D vector
pub fn results(y: &Vec<f64>) -> Box<Vec<f64>> {
    let mut max: f64;
    let mut min: f64;
    let mut result: Vec<f64> = Vec::new();
    let mut sum: f64 = 0.0;

    /* Set max and min for feature */
    max = y[0];
    min = y[0];

    /* Find max and min for feature */
    for i in y.iter() {
        if max < *i {
            max = *i;
        } else if min > *i {
            min = *i;
        } else {
            // do nothing
        }
        sum += *i;
    }

    let mean = sum / y.len() as f64;

    sum = 0.0;
    for i in y.iter() {
        sum += (*i - mean) * (*i - mean);
    }

    let std_deviation = (sum / y.len() as f64).sqrt();

    for i in y.iter() {
        result.push((*i - mean) / std_deviation);
    }

    Box::new(result)
}
