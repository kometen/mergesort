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
    let middle = v_len/2;
    let mut max_size = 1;
    println!("size: {}, max-width: {}", &v_len, &middle);

    let mut width = 1;
    let mut left;
    let mut right;

    let mut i = 0;
    loop {
        //println!("width: {}", _w);
        left = i;
        print!("index: {}, left: {}, ", i, &numbers_vector[left]);
        i += 1;
        if i >= *v_len {
            break;
        }
        right = i;
        println!("index: {}, right: {}", i, &numbers_vector[right]);
        i += 1;
        if &numbers_vector[left] > &numbers_vector[right] {
            println!("swap {} and {}", &numbers_vector[left], &numbers_vector[right]);
            let tmp = numbers_vector[left];
            numbers_vector[left] = numbers_vector[right];
            numbers_vector[right] = tmp;
        }
        if i >= *v_len {
            println!("done");
            break;
        }
    }

    let mut first_element = true;
    print!("\nfirst sort: ");
    for n in &numbers_vector {
        if first_element == true {
            print!("{}", n);
            first_element = false;
        } else {
            print!(", {}", n);
        }
    }
    println!();
}
