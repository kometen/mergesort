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

    println!("size: {}", &v_len);

    let mut sorted = true;
    let mut width = 1;
    let mut left_pointer;
    let mut right_pointer;

    // First loop defines width, which is the space between left and right pointer.
    // Second loop will compare elements and swap if needed.
    loop {
        println!("\nwidth: {}", width);

        left_pointer = 0;

        loop {
            //print!("index: {}, left: {}, ", left_pointer, &numbers_vector[left_pointer]);
            right_pointer = left_pointer + width;

            if right_pointer >= *v_len {
                break;
            }

            //print!("index: {}, right: {}", right_pointer, &numbers_vector[right_pointer]);

            let mut swapped = false;
            let mut x = 0;
            let mut left = left_pointer;
            let mut right = right_pointer;
            loop {
                if right >= *v_len {
                    break;
                }
                print!("left: {}, right: {}, ", &numbers_vector[left], &numbers_vector[right]);
                if &numbers_vector[left] > &numbers_vector[right] {
                    sorted = false;
                    swapped = true;
                    println!(", swap {} and {}", &numbers_vector[left], &numbers_vector[right]);
                    let tmp = numbers_vector[left];
                    numbers_vector[left] = numbers_vector[right];
                    numbers_vector[right] = tmp;
                }
                x += 1;
                if (x >= width && swapped == false) || (right >= width * 2 && swapped == true) {
                    break;
                }
                left += 1;
                if left == right {
                    right += 1;
                }
            }

            println!();

            left_pointer = right_pointer + width;
        }

        if &width > v_len {
            println!("Reached halfway through the list, we should be done sorting");
            break;
        }

        width = width * 2;

    }

    if sorted == true {
        println!("Already sorted");
        exit(0);
    }

    let mut first_element = true;
    print!("\nsorted: ");
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
