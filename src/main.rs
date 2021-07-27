extern crate clap;

use clap::{Arg, App};
use std::process::exit;

fn main() {
    let mut numbers_vector: Vec<i32> = Vec::new();

    let matches = App::new("mergesort")
        .version("0.1")
        .about("mergesort")
        .author("Claus Guttesen")
        .arg(Arg::with_name("numbers")
            .short("n")
            .long("numbers")
            .takes_value(true)
            .required(true)
            .multiple(true)
        )
        .get_matches();

    if let Some(numbers) = matches.values_of("numbers") {
        for ns in numbers {
            let n = match ns.parse::<i32>() {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("exiting, reason: {}", e);
                    exit(-1);
                }
            };
            numbers_vector.push(n);
        }
    }

    let v_len = &numbers_vector.len();
    let max_width = v_len/2;
    println!("size: {}, max-width: {}", &v_len, &max_width);

//    let mut width;
//    let mut left;
    for n in &numbers_vector {
        println!("number: {}", n);
    }

}
