extern crate clap;

use clap::{Arg, App};
use std::process::exit;

fn msb(n: usize) -> usize {
    if n == 0 { return 0; }

    let mut msb = 0;
    let mut n = n;
    n = n / 2;
    while n != 0 {
        n = n / 2;
        msb = msb + 1;
    }

    return 1 << msb;
}

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
    let most_significant_bit = msb(*v_len);

    println!("size: {}, most significant bit: {}", &v_len, &most_significant_bit);

    let mut sorted = true;
    let mut width = 1;
    let mut left_pointer;
    let mut right_pointer;

    // First loop defines width, which is the space between left and right pointer.
    // Second loop will compare elements and swap if needed.
    loop {
        if width > most_significant_bit {
            break;
        }

        println!("\nwidth: {}", width);

        left_pointer = 0;

        loop {
            print!("index: {}, left: {}, ", left_pointer, &numbers_vector[left_pointer]);
            right_pointer = left_pointer + width;

            if right_pointer >= *v_len {
                break;
            }

            print!("index: {}, right: {}", right_pointer, &numbers_vector[right_pointer]);

            if &numbers_vector[left_pointer] > &numbers_vector[right_pointer] {
                sorted = false;
                print!(", swap {} and {}", &numbers_vector[left_pointer], &numbers_vector[right_pointer]);
                let tmp = numbers_vector[left_pointer];
                numbers_vector[left_pointer] = numbers_vector[right_pointer];
                numbers_vector[right_pointer] = tmp;
            }

            println!();

            left_pointer = right_pointer + width;
        }

        width = width * 2;
    }

    if sorted == true {
        println!("Already sorted");
        exit(0);
    }

    let mut first_element = true;
    print!("\nsort: ");
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
