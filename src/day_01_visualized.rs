use ansi_term::Color::{Red, RGB};
use std::fs::File;
use std::io::Read;
use plotters::prelude::*;

const COLOR_PALETTE: &[RGBColor] = &[
    RGBColor(244, 242, 241),
    RGBColor(201, 192, 187),
    RGBColor(204, 102, 102),
    RGBColor(86, 136, 125),
    RGBColor(160, 120, 90)
];

pub fn day_01_main(with_plot: bool) {
    println!(
        "{}\n\t• {} is the highest calory number\n\t• {} are the highest three calory numbers summed up",
        RGB(204, 204, 0)
            .underline()
            .paint("These are the results for day 1:"),
        Red.paint(both_parts_01(with_plot).0.to_string()),
        Red.paint(both_parts_01(false).1.to_string())
    )
}

fn both_parts_01(with_plot: bool) -> (i32, i32) {
    let file = File::open("assets/01/calories_list.txt");
    let mut file = file.expect("Something went wrong reading the input file");

    let mut calories_list = String::new();
    file.read_to_string(&mut calories_list)
        .expect("Something went wrong writing input to vec");

    let mut single_elve;
    let mut elves_calories_split: Vec<Vec<i32>> = Vec::new();
    let mut elves_calories: Vec<i32> = Vec::new();

    for elve_collection in calories_list.split("\r\n\r\n").enumerate() {
        single_elve = 0;

        for calory in elve_collection.1.split('\n') {
            single_elve += calory.trim().parse::<i32>().unwrap();
        } 

        let temp_index = match elves_calories.binary_search(&single_elve) {
            Ok(x) =>  {
                elves_calories.insert(x, single_elve);
                x
            },
            Err(x) => {
                elves_calories.insert(x, single_elve);
                x
            },
        };

        let mut temp_list = Vec::new();

        for element in elve_collection.1.split('\n') {
            temp_list.push(element.trim().parse::<i32>().unwrap())
        }

        elves_calories_split.insert(
            temp_index,
            temp_list
        );

    }

    if with_plot {
        create_plot(&elves_calories_split);
    }
    
    (
        elves_calories[elves_calories.len() - 1], 
        elves_calories[elves_calories.len() - 1] +
        elves_calories[elves_calories.len() - 2] +
        elves_calories[elves_calories.len() - 3]
    )
}

fn create_plot(plot_data: &[Vec<i32>]) {
    // create drawing area and fill it with grey
    let drawing_area = BitMapBackend::gif(
        "diagrams/day_01_fast.gif", 
        (1920, 1080), 
        1
    ).unwrap().into_drawing_area();
    drawing_area.fill(&RGBColor(61, 61, 61)).unwrap();

    // create chart on drawing area
    let mut chart = ChartBuilder::on(&drawing_area)
        .margin(50)
        .build_cartesian_2d(0..240, 0..70000)
        .unwrap();
    
    for (i, dataset) in plot_data.iter().enumerate() {
        chart.draw_series((0..).zip(dataset.iter().enumerate()).map(|(x, (j, y))| {
            match j {
                0 => {
                    drawing_area.present().unwrap();

                    Rectangle::new([
                            (i as i32, 0),
                            ((i as i32) + 1, *y)
                        ], COLOR_PALETTE[ (x as usize) % 5].filled()
                    )
                }
                _ => {
                    drawing_area.present().unwrap();

                    let mut height = 0;
                    for (limit, element_height) in dataset.iter().enumerate() {
                        if limit <= (j - 1) {
                            height += element_height;
                        } else {
                            break;
                        } 

                    }
                    Rectangle::new([
                            (i as i32, dataset[j - 1]),
                            ((i as i32) + 1, height + *y)
                        ], COLOR_PALETTE[ (x as usize) % 5].filled()
                    )
                }
            }
        }))
        .unwrap();
    }
}
