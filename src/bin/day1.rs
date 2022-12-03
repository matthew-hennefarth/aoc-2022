fn main() {

    aoc_2022::print_header(1);
    let input = aoc_2022::get_input();
    
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
            calories += (*line).parse::<usize>().unwrap();
        }
    }
    println!("Max calories is:             \x1b[93m{max_calories:>8}\x1b[0m");

    let mut top_calories: [usize; 3] = [0; 3];
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
            calories += line.parse::<usize>().unwrap();
        }
    }

    let sum: usize = top_calories.iter().sum();
    println!("Calories of top 3 elves is:  \x1b[93m{sum:>8}\x1b[0m");
    println!("");

}
