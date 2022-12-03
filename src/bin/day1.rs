use std::io::{self, BufRead};

fn get_input() -> Vec<String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();
    let mut eof = false;
    let mut lines = Vec::new();

    while !eof {
        match handle.read_line(&mut buffer) {
            Ok(0) => eof = true,
            Ok(_) => {
                buffer.pop();
                lines.push(buffer);
                buffer = String::new();
            },
            Err(_error) => panic!("Something went wrong reading in!")
        }
    }

    lines
}

fn main() {

    println!("==================");
    println!("||    DAY 01    ||");
    println!("==================");

    let input = get_input();
    
    let mut calories = 0;
    let mut max_calories = 0;

    for line in &input {
        if line.is_empty() {
            if calories > max_calories {
                max_calories = calories;
            }
            calories = 0;
        }
        else {
            calories += (*line).parse::<isize>().unwrap();
        }
    }
    println!("Max calories is: {}", max_calories);

    let mut top_calories = vec![0, 0, 0];
    calories = 0;

    for line in input {
        if line.is_empty() {
            if calories > top_calories[0] {
                top_calories[0] = calories;
                top_calories.sort();
            } 
            calories = 0;
        }
        else {
            calories += line.parse::<isize>().unwrap();
        }
    }

    let sum: isize = top_calories.iter().sum();
    println!("Calories of top 3 elves is: {}", sum);
    println!("");

}
