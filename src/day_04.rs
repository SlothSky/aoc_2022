use ansi_term::Color::{Red, RGB};
use std::fs::File;
use std::io::Read;

pub fn day_04_main() {
    println!(
        "{}\n\t• {} complete overlaps are present\n\t• {} overlaps are present",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 4:"),
        Red.paint(first_part_04().to_string()),
        Red.paint(second_part_04().to_string())
    )
}

fn input_function(day: &str) -> String {
    let path = format!("assets/{}/input.txt", &day);

    let file = File::open(path);
    let mut file = file.expect("Something went wrong reading the input file");

    let mut input_list = String::new();
    file.read_to_string(&mut input_list)
        .expect("Something went wrong writing input to vec");

    input_list
}

fn first_part_04() -> i32 {
    let assignment_pairs = input_function("04");

    let mut overlaps = 0;    
    for pair in assignment_pairs.split('\n') {
        let elves_pairs: Vec<&str> = pair.split(',').collect();

        match &elves_pairs[0].split('-').collect::<Vec<_>>()[0].trim().parse::<i32>().unwrap().cmp(
            &elves_pairs[1].split('-').collect::<Vec<_>>()[0].trim().parse::<i32>().unwrap()) {

            std::cmp::Ordering::Less => {
                match &elves_pairs[0].split('-').collect::<Vec<_>>()[1].trim().parse::<i32>().unwrap().cmp(
                    &elves_pairs[1].split('-').collect::<Vec<_>>()[1].trim().parse::<i32>().unwrap()) {
        
                        std::cmp::Ordering::Less => (),
                        std::cmp::Ordering::Equal => {
                            overlaps += 1;
                        },
                        std::cmp::Ordering::Greater => {
                            overlaps += 1;
                        },
                }
            },
            std::cmp::Ordering::Equal => overlaps += 1,
            std::cmp::Ordering::Greater => {
                match &elves_pairs[0].split('-').collect::<Vec<_>>()[1].trim().parse::<i32>().unwrap().cmp(
                    &elves_pairs[1].split('-').collect::<Vec<_>>()[1].trim().parse::<i32>().unwrap()) {
        
                        std::cmp::Ordering::Less => {
                            overlaps += 1;
                        },
                        std::cmp::Ordering::Equal => {
                            overlaps += 1;
                        },
                        std::cmp::Ordering::Greater => (),
                }
            },
        }
    }

    overlaps
}

fn second_part_04() -> i32 {
    let assignment_pairs = input_function("04");

    let mut overlaps = 0;    
    for pair in assignment_pairs.split('\n') {
        let elves_pairs: Vec<&str> = pair.split(',').collect();

        match &elves_pairs[0].split('-').collect::<Vec<_>>()[1].trim().parse::<i32>().unwrap().cmp(
            &elves_pairs[1].split('-').collect::<Vec<_>>()[0].trim().parse::<i32>().unwrap()) {

            std::cmp::Ordering::Less => (),
            std::cmp::Ordering::Equal => { 
                match &elves_pairs[0].split('-').collect::<Vec<_>>()[0].trim().parse::<i32>().unwrap().cmp(
                    &elves_pairs[1].split('-').collect::<Vec<_>>()[1].trim().parse::<i32>().unwrap()) {
                        std::cmp::Ordering::Less => {
                            overlaps += 1;
                            continue;
                        },
                        std::cmp::Ordering::Equal => {
                            overlaps += 1;
                            continue;
                        },
                        std::cmp::Ordering::Greater => (),
                };
            },
            std::cmp::Ordering::Greater => { 
                match &elves_pairs[0].split('-').collect::<Vec<_>>()[0].trim().parse::<i32>().unwrap().cmp(
                    &elves_pairs[1].split('-').collect::<Vec<_>>()[1].trim().parse::<i32>().unwrap()) {
                        std::cmp::Ordering::Less => {
                            overlaps += 1;
                            continue;
                        },
                        std::cmp::Ordering::Equal => {
                            overlaps += 1;
                            continue;
                        },
                        std::cmp::Ordering::Greater => (),
                };
            },
        };

        match &elves_pairs[0].split('-').collect::<Vec<_>>()[0].trim().parse::<i32>().unwrap().cmp(
            &elves_pairs[1].split('-').collect::<Vec<_>>()[1].trim().parse::<i32>().unwrap()) {

            std::cmp::Ordering::Less => {
                match &elves_pairs[0].split('-').collect::<Vec<_>>()[1].trim().parse::<i32>().unwrap().cmp(
                    &elves_pairs[1].split('-').collect::<Vec<_>>()[0].trim().parse::<i32>().unwrap()) {
                        std::cmp::Ordering::Less => (),
                        std::cmp::Ordering::Equal => {
                            overlaps += 1;
                            continue;
                        },
                        std::cmp::Ordering::Greater => {
                            overlaps += 1;
                            continue;
                        },
                };
            },
            std::cmp::Ordering::Equal => {
                match &elves_pairs[0].split('-').collect::<Vec<_>>()[1].trim().parse::<i32>().unwrap().cmp(
                    &elves_pairs[1].split('-').collect::<Vec<_>>()[0].trim().parse::<i32>().unwrap()) {
                        std::cmp::Ordering::Less => (),
                        std::cmp::Ordering::Equal => {
                            overlaps += 1;
                            continue;
                        },
                        std::cmp::Ordering::Greater => {
                            overlaps += 1;
                            continue;
                        },
                };
            },
            std::cmp::Ordering::Greater => (), 
        };
    }

    overlaps
}
