use std::env;
pub use std::path::Path;

use feature_scaling::{features, file_ops, results};

fn handle_input(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Error: no input argument");
        std::process::exit(exitcode::USAGE);
    }

    if args[1].is_empty() {
        eprintln!("Error: filename is empty");
        std::process::exit(exitcode::USAGE);
    }

    if args[2].is_empty() {
        eprintln!("Error: empty flag");
        std::process::exit(exitcode::USAGE);
    }

    if args[2] != "-o" {
        eprint!("Error: unknow flag");
        std::process::exit(exitcode::USAGE);
    }

    if args[3].is_empty() {
        eprintln!("Error in output filename");
        std::process::exit(exitcode::USAGE);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // check the input arguments
    handle_input(&args);

    let file_path = Path::new(&args[1]);
    let output_path = Path::new(&args[1]);

    let (x_ptr, y_ptr) = match file_ops::get_data(file_path) {
        Ok((x_ptr, y_ptr)) => (x_ptr, y_ptr),
        Err(e) => {
            eprintln!("{}", e.get_ref().unwrap());
            std::process::exit(exitcode::IOERR);
        }
    };

    let x = *x_ptr;
    let y = *y_ptr;

    let result_x = features::mean_normal(&x);
    let result_y = results::mean_normal(&y);

    file_ops::write_data(&result_x, &result_y, output_path);

    for (i, x_set) in result_x.iter().enumerate() {
        for x_v in x_set.iter() {
            print!("{}, ", x_v);
        }
        println!("{}", result_y[i]);
    }
}
