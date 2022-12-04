fn get_range(assignment: &str) -> [usize; 2] {
    let mut result = [0; 2];
    for (index, num) in assignment.split('-').enumerate() {
        result[index] = num.parse::<usize>().unwrap(); 
    }
    result
}

fn range_contains(range1: [usize; 2], range2: [usize; 2]) -> bool {
    if range1[0] <= range2[0] && range2[1] <= range1[1] {
        return true;
    }
    false
}

fn overlaps(range1: [usize; 2], range2: [usize; 2]) -> bool {
    if range1[0] <= range2[0] && range2[0] <= range1[1] {
        return true;
    }

    if range1[0] <= range2[1] && range2[1] <= range1[1] {
        return true;
    }
    false
}

fn ranges_overlaps(range1: [usize; 2], range2: [usize; 2]) -> bool {
    if range_contains(range1, range2) || range_contains(range2, range1){
        return true;
    } 

    if overlaps(range1, range2) || overlaps(range2, range1) {
        return true;
    }

    false
}

fn main() {

    aoc_2022::print_header(1);
    let input = aoc_2022::get_input();
    
    let mut fully_contains = 0;
    let mut overlaps = 0;
    for line in &input {
        let assignments = line.split(',').collect::<Vec<&str>>();
        assert!(assignments.len() == 2);

        let elf_1 = get_range(assignments[0]);
        let elf_2 = get_range(assignments[1]);

        if range_contains(elf_1, elf_2) || range_contains(elf_2, elf_1) {
            fully_contains += 1;
        }

        if ranges_overlaps(elf_1, elf_2) {
            overlaps += 1;
        }


    }
    println!("Number of duplicates:        \x1b[93m{fully_contains:>8}\x1b[0m");
    println!("Number of overlaps:          \x1b[93m{overlaps:>8}\x1b[0m");
    println!("");

}
