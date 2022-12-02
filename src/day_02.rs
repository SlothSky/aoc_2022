use ansi_term::Color::{Red, RGB};
use std::fs::File;
use std::io::Read;

const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";

const OWN_ROCK: &str = "X";
const OWN_PAPER: &str = "Y";
const OWN_SCISSORS: &str = "Z";

const LOOSE: &str = "X";
const DRAW: &str = "Y";
const WIN: &str = "Z";

const DRAW_VALUE: i32 = 3;
const WIN_VALUE: i32 = 6;

const ROCK_VALUE: i32 = 1;
const PAPER_VALUE: i32 = 2;
const SCISSORS_VALUE: i32 = 3;

pub fn day_02_main() {
    println!(
        "{}\n\t• {} is the score \n\t• {} is the new strategy's score",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 2:"),
        Red.paint(both_parts_02(0).0.to_string()),
        Red.paint(both_parts_02(1).1.to_string())
    )
}

fn both_parts_02(variant: u8) -> (i32, i32) {
    let file = File::open("assets/02/strategy_guide.txt");
    let mut file = file.expect("Something went wrong reading the input file");

    let mut strategy_guide = String::new();
    file.read_to_string(&mut strategy_guide)
        .expect("Something went wrong writing input to vec");

    let mut shape_score = 0;
    let mut result_score = 0;

    for strategy in strategy_guide.split('\n') {
        let strategy_pair: Vec<&str> = strategy.split(' ').collect();

        match variant {
            0 => {
                match strategy_pair[1].trim() {
                    OWN_ROCK => { 
                        shape_score += ROCK_VALUE;

                        match strategy_pair[0].trim() {
                            OPPONENT_ROCK => result_score += DRAW_VALUE,
                            OPPONENT_SCISSORS => result_score += WIN_VALUE,
                            _ => (),
                        }
                    },
                    OWN_PAPER => { 
                        shape_score += PAPER_VALUE;

                        match strategy_pair[0].trim() {
                            OPPONENT_PAPER => result_score += DRAW_VALUE,
                            OPPONENT_ROCK => result_score += WIN_VALUE,
                            _ => (),
                        }
                    },
                    OWN_SCISSORS => { 
                        shape_score += SCISSORS_VALUE;

                        match strategy_pair[0].trim() {
                            OPPONENT_SCISSORS => result_score += DRAW_VALUE,
                            OPPONENT_PAPER => result_score += WIN_VALUE,
                            _ => (),
                        }
                    },
                    _ => (),
                }
            },
            1 => {
                match strategy_pair[0].trim() {
                    OPPONENT_ROCK => { 

                        match strategy_pair[1].trim() {
                            WIN => result_score += WIN_VALUE + PAPER_VALUE,
                            DRAW => result_score += DRAW_VALUE + ROCK_VALUE,
                            LOOSE => result_score += SCISSORS_VALUE,
                            _ => (),
                        }
                    },
                    OPPONENT_PAPER => { 

                        match strategy_pair[1].trim() {
                            WIN => result_score += WIN_VALUE + SCISSORS_VALUE,
                            DRAW => result_score += DRAW_VALUE + PAPER_VALUE,
                            LOOSE => result_score += 1,
                            _ => (),
                        }
                    },
                    OPPONENT_SCISSORS => { 

                        match strategy_pair[1].trim() {
                            WIN => result_score += WIN_VALUE + ROCK_VALUE,
                            DRAW => result_score += DRAW_VALUE + SCISSORS_VALUE,
                            LOOSE => result_score += PAPER_VALUE,
                            _ => (),
                        }
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }
    
    (
        shape_score + result_score,
        result_score
    )

}
