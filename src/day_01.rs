use ansi_term::Color::{Red, RGB};
use std::fs::File;
use std::io::Read;

pub fn day_01_main() {
    println!(
        "{}\n\t• {} is the highest calory number\n\t• {} are the highest three calory numbers summed up",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 1:"),
        Red.paint(both_parts_01().0.to_string()),
        Red.paint(both_parts_01().1.to_string())
    )
}

fn both_parts_01() -> (i32, i32) {
    let file = File::open("assets/01/calories_list.txt");
    let mut file = file.expect("Something went wrong reading the input file");

    let mut calories_list = String::new();
    file.read_to_string(&mut calories_list)
        .expect("Something went wrong writing input to vec");

    let mut single_elve;
    let mut elves_calories: Vec<i32> = Vec::new();

    for elve_collection in calories_list.split("\r\n\r\n").enumerate() {
        single_elve = 0;

        for calory in elve_collection.1.split('\n') {
            single_elve += calory.trim().parse::<i32>().unwrap();
        } 

        match elves_calories.binary_search(&single_elve) {
            Ok(x) => elves_calories.insert(x, single_elve),
            Err(x) => elves_calories.insert(x, single_elve),
        }
    }
    
    (
        elves_calories[elves_calories.len() - 1], 
        elves_calories[elves_calories.len() - 1] +
        elves_calories[elves_calories.len() - 2] +
        elves_calories[elves_calories.len() - 3]
    )

}
