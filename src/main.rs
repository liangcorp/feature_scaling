use std::path::Path;
use std::{env, io, thread};

use feature_scaling::{file_ops, mean_normal};

const ARGS_SIZE_LIMIT: usize = 4;

fn argument_check(args: &[String]) -> Result<(), io::Error> {
    let mut error = String::new();

    if args.len() < ARGS_SIZE_LIMIT {
        error = String::from("not enough input argument");
    } else if args[1].is_empty() {
        error = String::from("filename is empty");
    } else if args[2].is_empty() {
        error = String::from("empty flag");
    } else if args[2] != "-o" {
        error = String::from("unknow flag");
    } else if args[3].is_empty() {
        error = String::from("output filename");
    }

    if error.is_empty() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, error))
    }
}

fn display_help(err: io::Error) {
    eprintln!("ERROR: {}\n", err);
    let help_message = String::from(
        "Usage: INPUT -o OUTPUT
Mean normalization of data from file <INPUT> into file <OUTPUT>
               \t-o OUTPUT\tSpecify the output filename for result data\n",
    );
    println!("{}", help_message);
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // check the input arguments
    match argument_check(&args) {
        Ok(_) => print!("Reading data from path: "),
        Err(e) => display_help(e),
    };

    //  Read data from file into vectors pointers
    let input_file_path = Path::new(&args[1]);
    print!("./{}... ", input_file_path.to_str().unwrap());

    let (x_ptr, y_ptr) = match file_ops::read_data(input_file_path) {
        Ok((x_ptr, y_ptr)) => {
            println!("OK");
            (x_ptr, y_ptr)
        }
        Err(e) => {
            eprintln!("{}", e.get_ref().unwrap());
            std::process::exit(1);
        }
    };

    // Spawn thread for mean normalization on X (2D)
    let thread_join_handle = thread::spawn(move || mean_normal::features(&x_ptr.to_vec()));

    // Mean normalization on y (1D)
    let result_y = mean_normal::results(&y_ptr);

    //  Get result from X mean normalization thread
    let result_x = thread_join_handle.join().ok().unwrap();

    //  Write to file
    let output_file_path = Path::new(&args[3]);
    print!("Writting data to file... ");

    match file_ops::write_data(&result_x, &result_y, output_file_path) {
        Ok(_) => println!("OK"),
        Err(e) => {
            eprintln!("{}", e.get_ref().unwrap());
            std::process::exit(1);
        }
    }
}
