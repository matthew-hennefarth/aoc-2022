use std::io::{self, BufRead};

pub fn get_input() -> Vec<String> {
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

pub fn print_header(day: usize) {
    println!("\x1b[91m==================\x1b[0m");
    println!("\x1b[92m\x1b[1m||    DAY \x1b[91m{day:0>2}\x1b[92m    ||\x1b[0m");
    println!("\x1b[91m==================\x1b[0m");
}
