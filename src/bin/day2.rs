#[derive(Debug, PartialEq)]
enum Result {
    Win = 6,
    Tie = 3,
    Loss = 0
}

impl Result {
    fn from_char(c: char) -> Self {
        match c {
            'X' => Result::Loss,
            'Y' => Result::Tie,
            'Z' => Result::Win,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Decision {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl Decision {
    fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => Decision::Rock,
            'B' | 'Y' => Decision::Paper,
            'C' | 'Z' => Decision::Scissors,
            _ => unreachable!()
        }
    }

    fn next(&self) -> Self {
        match *self {
            Decision::Rock => Decision::Paper,
            Decision::Paper => Decision::Scissors,
            Decision::Scissors => Decision::Rock
        }
    }

    fn play(&self, other_player: &Decision) -> Result {
        if self == other_player {
            return Result::Tie;
        }
        else if self.next() == *other_player {
            return Result::Loss;
        }
        Result::Win
    }

    fn needed_decision(&self, result: &Result) -> Self {
        if result == &Result::Tie {
            return *self;
        }
        else if result == &Result::Win {
            return self.next();
        }
        self.next()
    }
}

fn main() {

    aoc_2022::print_header(2);
    let input = aoc_2022::get_input();

    let mut total_points = 0;

    for line in &input {
        let p1 = (*line).as_bytes()[0] as char;
        let p2 = (*line).as_bytes()[2] as char;
    
        let p1 = Decision::from_char(p1);
        let p2 = Decision::from_char(p2);

        let result = p2.play(&p1);
        
        total_points += (result as usize) + (p2 as usize);

    }

    println!("Total points:                   \x1b[93m{total_points:>4}\x1b[0m");


    total_points = 0;
    for line in &input {
        let p1 = line.as_bytes()[0] as char;
        let result = line.as_bytes()[2] as char;

        let p1 = Decision::from_char(p1);
        let result = Result::from_char(result);
    
        let p2 = p1.needed_decision(&result);
        total_points += (result as usize) + (p2 as usize);

    }

    println!("Total points with new strategy: \x1b[93m{total_points:>4}\x1b[0m");
    println!("");
}
