use std::collections::HashSet;

fn get_matching_char(sack1: &str, sack2: &str) -> char {
    let mut prev1 = HashSet::new();
    let mut prev2 = HashSet::new();

    for (c1, c2) in sack1.chars().zip(sack2.chars()) {
        if c1 == c2 {
            return c1;
        }
        if prev2.contains(&c1) {
            return c1;
        }
        if prev1.contains(&c2) {
            return c2;
        }
        prev1.insert(c1);
        prev2.insert(c2);
    }
    unreachable!();
}

fn get_matching_char_3(sacks: &[String]) -> char {
    
    let mut prev1 = HashSet::new();
    let mut prev2 = HashSet::new();
    let mut prev3 = HashSet::new();

    let sack1 = &sacks[0];
    let sack2 = &sacks[1];
    let sack3 = &sacks[2];

    let mut index1 = 0;
    let mut index2 = 0;
    let mut index3 = 0;

    while index1 < sack1.len() - 1  || index2 < sack2.len() - 1 || index3 < sack3.len() - 1 {
        let c1 = sack1.chars().nth(index1).unwrap();
        let c2 = sack2.chars().nth(index2).unwrap();
        let c3 = sack3.chars().nth(index3).unwrap();
       
        if c1 == c2 && c1 == c3 {
            return c1;
        }

        if c1 == c2 && prev3.contains(&c1) {
            return c1;
        }

        if c1 == c3 && prev2.contains(&c1) {
            return c1;
        }

        if c2 == c3 && prev1.contains(&c2) {
            return c2;
        }

        if prev2.contains(&c1) && prev3.contains(&c1) {
            return c1;
        }
        if prev1.contains(&c2) && prev3.contains(&c2) {
            return c2;
        }
        if prev1.contains(&c3) && prev2.contains(&c3) {
            return c3;
        }
        prev1.insert(c1);
        prev2.insert(c2);
        prev3.insert(c3);

        if index1 < sack1.len() - 1 {
            index1 += 1;
        } 
        if index2 < sack2.len() - 1 {
            index2 += 1;
        } 
        if index3 < sack3.len() - 1 {
            index3 += 1;
        } 
 
    }

    let c1 = sack1.chars().nth(index1).unwrap();
    let c2 = sack2.chars().nth(index2).unwrap();
    let c3 = sack3.chars().nth(index3).unwrap();
    
    if c1 == c2 && c1 == c3 {
        return c1;
    }

    if prev2.contains(&c1) && prev3.contains(&c1) {
        return c1;
    }
    if prev1.contains(&c2) && prev3.contains(&c2) {
        return c2;
    }
    if prev1.contains(&c3) && prev2.contains(&c3) {
        return c3;
    }

    unreachable!();
}

fn get_importance(c: char) -> usize {
    if c.is_lowercase() {
        return ((c as u8) - b'a') as usize + 1;
    }
    else {
        return ((c as u8) - b'A') as usize + 27;
    }
}

fn main() {

    aoc_2022::print_header(3);
    let input = aoc_2022::get_input();

    let mut total = 0;
    for line in &input {
        let len = line.len();
        let sack1 = &line[..len/2];
        let sack2 = &line[len/2..];

        let matching = get_matching_char(sack1, sack2);
        let importance = get_importance(matching);

        total += importance;
    }

    println!("Sum of priorities for items:  \x1b[93m{total:>6}\x1b[0m");

    total = 0;

    for sacks in input.chunks(3) {
        let matching = get_matching_char_3(sacks);
        let importance = get_importance(matching);
        total += importance;
    }

    println!("Sum of priorities for badges: \x1b[93m{total:>6}\x1b[0m");
    println!("");

}
