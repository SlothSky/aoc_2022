use ansi_term::Color::{Red, RGB};
use std::fs::File;
use std::io::Read;
use std::vec;


pub fn day_05_main() {
    println!(
        "{}\n\t• {} complete overlaps are present\n\t• {} overlaps are present",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 5:"),
        Red.paint(first_part_05().to_string()),
        Red.paint(second_part_05().to_string())
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

fn first_part_05() -> i32 {
    let mut stack_1: Vec<char> = vec!['C','Z','N','B','M','W','Q','V'];
    let mut stack_2: Vec<char> = vec!['H','Z','R','W','C','B'];
    let mut stack_3: Vec<char> = vec!['F','Q','R','J'];
    let mut stack_4: Vec<char> = vec!['Z', 'S', 'W', 'H', 'F', 'N', 'M', 'T'];
    let mut stack_5: Vec<char> = vec!['G','F','W','L','N','Q','P'];
    let mut stack_6: Vec<char> = vec!['L','P','W'];
    let mut stack_7: Vec<char> = vec!['V','B','D','R','G','C','Q','J'];
    let mut stack_8: Vec<char> = vec!['Z','Q','N','B','Q'];
    let mut stack_9: Vec<char> = vec!['H','L','F','C','G','T','J'];
 
    let move_list = input_function("05");

    let mut overlaps = 0;    
    for movement in move_list.split('\n') {
        let mo_size = movement.split('-').collect::<Vec<&str>>()[0].trim().parse::<i32>().unwrap();
        let start_stack = movement.split('-').collect::<Vec<&str>>()[1].split('|').
                                collect::<Vec<&str>>()[0].trim().parse::<usize>().unwrap();
        let target_stack = movement.split('-').collect::<Vec<&str>>()[1].split('|').
                                collect::<Vec<&str>>()[1].trim().parse::<usize>().unwrap();
    

        
    }

    overlaps
}

fn second_part_05() -> i32 {
    let assignment_pairs = input_function("05");

    let mut overlaps = 0;    
    for pair in assignment_pairs.split('\n') {
        
    }

    overlaps
}
