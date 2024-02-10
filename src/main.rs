use std::env;
pub use std::path::Path;

use feature_scaling::{features, read_data, results};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: no input argument");
        std::process::exit(exitcode::DATAERR);
    }

    if args[1].is_empty() {
        eprintln!("Error: filename is empty");
        std::process::exit(exitcode::DATAERR);
    }

    let file_path = Path::new(&args[1]);

    let (x_ptr, y_ptr) = match read_data::get_data(file_path) {
        Ok((x_ptr, y_ptr)) => (x_ptr, y_ptr),
        Err(e) => {
            eprintln!("{}", e.get_ref().unwrap());
            std::process::exit(exitcode::IOERR);
        }
    };

    let x = *x_ptr;
    let y = *y_ptr;

    let (ex_2_nor_y, y_mean, y_std_dev) = results::mean_normal(&y);

    let (ex_2_nor_x, x_mean, x_std_dev) = features::mean_normal(&x);

    println!("{:?} {:?} {:?}", ex_2_nor_x, x_mean, x_std_dev);
    println!("{:?} {} {}", ex_2_nor_y, y_mean, y_std_dev);
}
