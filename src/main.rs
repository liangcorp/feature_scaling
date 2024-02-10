use std::env;
use std::path::Path;

use feature_scaling::{file_ops, mean_normal};

fn argument_check(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Error: not enough input argument");
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
    argument_check(&args);

    let input_file_path = Path::new(&args[1]);
    let output_file_path = Path::new(&args[3]);

    //  Read data from file into vectors pointers
    let (x_ptr, y_ptr) = match file_ops::read_data(input_file_path) {
        Ok((x_ptr, y_ptr)) => (x_ptr, y_ptr),
        Err(e) => {
            eprintln!("{}", e.get_ref().unwrap());
            std::process::exit(exitcode::IOERR);
        }
    };

    let result_x = mean_normal::features(&x_ptr.to_vec());
    let result_y = mean_normal::results(&y_ptr);

    //  Write to file
    match file_ops::write_data(&result_x, &result_y, output_file_path) {
        Ok(_) => println!("Data written to file"),
        Err(e) => {
            eprintln!("{}", e.get_ref().unwrap());
            std::process::exit(exitcode::IOERR);
        }
    }
}
