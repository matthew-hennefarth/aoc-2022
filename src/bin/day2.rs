
fn get_input() -> Vec<String> {
    use std::io::{self, BufRead};
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

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Decision {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl Decision {
    fn next(&self) -> Self {
        match *self {
            Decision::Rock => Decision::Paper,
            Decision::Paper => Decision::Scissors,
            Decision::Scissors => Decision::Rock
        }
    }

    fn prev(&self) -> Self {
        match *self {
            Decision::Rock => Decision::Scissors,
            Decision::Paper => Decision::Rock,
            Decision::Scissors => Decision::Paper
        }
    }
}

fn get_decision(choice: char) -> Decision {
    match choice {
        'A' | 'X' => Decision::Rock,
        'B' | 'Y' => Decision::Paper,
        'C' | 'Z' => Decision::Scissors,
        _ => unreachable!()
    }
}

#[derive(Debug, PartialEq)]
enum Result {
    Win = 6,
    Tie = 3,
    Loss = 0
}

fn get_result(choice: char) -> Result {
    match choice {
        'X' => Result::Loss,
        'Y' => Result::Tie,
        'Z' => Result::Win,
        _ => unreachable!()
    }
}

fn determine_result(p1: &Decision, p2: &Decision) -> Result {
    if p1 == p2 {
        return Result::Tie;
    }
    
    if *p1 == Decision::Rock && *p2 == Decision::Scissors {
        return Result::Loss;
    }
    if *p1 == Decision::Scissors && *p2 == Decision::Rock {
        return Result::Win;
    }
    if p1 < p2 {
        return Result::Win;
    }
    Result::Loss
}

fn determine_decision(p1: &Decision, result: &Result) -> Decision {
    if result == &Result::Tie {
        return *p1
    }
    else if result == &Result::Win {
        return p1.next();
    }
    p1.prev()
}

fn main() {

    println!("==================");
    println!("||    DAY 02    ||");
    println!("==================");

    let input = get_input();

    let mut total_points = 0;

    for line in &input {
        let p1 = (*line).as_bytes()[0] as char;
        let p2 = (*line).as_bytes()[2] as char;
    
        let p1 = get_decision(p1);
        let p2 = get_decision(p2);

        let result = determine_result(&p1, &p2);
        
        total_points += (result as usize) + (p2 as usize);

    }

    println!("Total points: {}", total_points);


    total_points = 0;
    for line in &input {
        let p1 = line.as_bytes()[0] as char;
        let result = line.as_bytes()[2] as char;

        let p1 = get_decision(p1);
        let result = get_result(result);
    
        let p2 = determine_decision(&p1, &result);
        total_points += (result as usize) + (p2 as usize);

    }

    println!("Total points with new strategy: {}", total_points);
    println!("");
}
