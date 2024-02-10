//! # Implementation of feature normalisation
///
/// Use mean normalization on 1D array.
/// This is used on Y that is a 1D array.
///
#[allow(dead_code)]
pub fn mean_normal_single(v: &Vec<f64>) -> (Box<Vec<f64>>, f64, f64) {
    let mut max: f64;
    let mut min: f64;
    let mut result: Vec<f64> = Vec::new();
    let mut sum: f64 = 0.0;

    /* Set max and min for feature */
    max = v[0];
    min = v[0];

    /* Find max and min for feature */
    for i in v.iter() {
        if max < *i {
            max = *i;
        } else if min > *i {
            min = *i;
        } else {
            // do nothing
        }
        sum += *i;
    }

    let mean = sum / v.len() as f64;

    sum = 0.0;
    for i in v.iter() {
        sum += (*i - mean) * (*i - mean);
    }

    let std_dev = (sum / v.len() as f64).sqrt();

    for i in v.iter() {
        result.push((*i - mean) / std_dev);
    }

    (Box::new(result), mean, std_dev)
}
