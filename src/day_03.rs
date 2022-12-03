use ansi_term::Color::{Red, RGB};
use std::fs::File;
use std::io::Read;

pub fn day_03_main() {
    println!(
        "{}\n\t• {} is the priority sum of the same elements\n\t• {} is the badge priority",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 3:"),
        Red.paint(first_part_03().to_string()),
        Red.paint(second_part_03().to_string())
    )
}

fn input_function(day: &str) -> String {
    let path = format!("assets/{}/rucksacks.txt", &day);

    let file = File::open(path);
    let mut file = file.expect("Something went wrong reading the input file");

    let mut rucksack_list = String::new();
    file.read_to_string(&mut rucksack_list)
        .expect("Something went wrong writing input to vec");

    rucksack_list
}

fn first_part_03() -> i32 {
    let rucksack_list = input_function("03");

    let mut priorities = 0;    
    for rucksack in rucksack_list.split('\n') {
        let (l_compartment, r_compartment) = rucksack.split_at(rucksack.len() / 2);

        for letter in l_compartment.as_bytes() {
            let letter_of_char = *letter as char;

            if r_compartment.contains(letter_of_char) {
                if letter_of_char.is_uppercase() {
                    // 38 is offset to uppercase char value
                    priorities += (letter - 38) as i32;
                } else {
                    // 96 is offset to lowercase char value
                    priorities += (letter - 96) as i32;
                }
                break;
            }
        }
    }

    priorities
}

fn second_part_03() -> i32 {
    let rucksack_list = input_function("03");

    let mut counter = 1;
    let mut current_string = Vec::new();
    let mut priorities = 0;

    for rucksack in rucksack_list.split("\r\n") {
        if counter % 3 == 0 {
            current_string.push(rucksack);

            for letter in current_string[0].as_bytes() {
                let second_badge_matches = current_string[1].contains(*letter as char);
                let third_badge_matches = current_string[2].contains(*letter as char);

                if second_badge_matches && third_badge_matches {
                    let letter_of_char = *letter as char;

                    if letter_of_char.is_uppercase() {
                        // 38 is offset to uppercase char value
                        priorities += (letter - 38) as i32;
                    } else {
                        // 96 is offset to lowercase char value
                        priorities += (letter - 96) as i32;
                    }

                    break;
                }
            } 
            current_string = Vec::new();

            counter += 1;
        } else {
            current_string.push(rucksack);

            counter += 1;
        }
    }

    priorities
}
