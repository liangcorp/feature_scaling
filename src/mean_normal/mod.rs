//! # Use mean normalization on 2D and 1D array.
//!

type DoubleVecF64 = Vec<Vec<f32>>;

/// Mean normalization for X.
/// X represent features (i.e. columns) which contains more than one field
/// Therefore, it is represented as a 2D vector
pub fn features(x: &[Vec<f32>]) -> Box<DoubleVecF64> {
    let mut vec_max: Vec<f32> = Vec::new();
    let mut vec_min: Vec<f32> = Vec::new();
    let mut vec_mean: Vec<f32> = Vec::new();

    let mut result: DoubleVecF64 = x.to_vec();

    let row = x.len();
    let col = x[0].len();

    let mut sum: f32;

    // Set default value max and min for each feature (i.e. column)
    // The goal is to expend the vectors to the size of
    // the number of features.
    for i in x[0].iter() {
        vec_max.push(*i);
        vec_min.push(*i);
    }

    // Find max, min and sum of each feature (i.e. column)
    // Each column is a feature, this means
    //  we need to loop from column to row.
    for j in 0..col {
        sum = 0.0;
        for x_row in x.iter() {
            if vec_max[j] < x_row[j] {
                vec_max[j] = x_row[j];
            }

            if vec_min[j] > x_row[j] {
                vec_min[j] = x_row[j];
            }

            sum += x_row[j];
        }

        // mean = sum of elements / no of elements
        vec_mean.push(sum / row as f32);

        for i in 0..row {
            //  Set the value of new 2D arry to normalized value
            result[i][j] = (x[i][j] - vec_mean[j]) / (vec_max[j] - vec_min[j]);
        }
    }

    Box::new(result)
}

/// Mean normalization for Y. Y only has one field.
/// It's represented as a 1D vector
pub fn results(y: &[f32]) -> Box<Vec<f32>> {
    let mut max: f32;
    let mut min: f32;
    let mut result: Vec<f32> = Vec::new();

    /* Set max and min for feature */
    max = y[0];
    min = y[0];

    /* Find max and min for feature */
    for element in y.iter() {
        if max < *element {
            max = *element;
        }
        if min > *element {
            min = *element;
        }
    }

    let sum: f32 = y.iter().sum();

    let mean = sum / y.len() as f32;

    for element in y.iter() {
        result.push((*element - mean) / (max - min));
    }

    Box::new(result)
}
