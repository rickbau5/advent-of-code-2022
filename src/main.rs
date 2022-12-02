use std::{fs, path::Path};

fn main() {
    print_solutions(run_day1());
}

fn print_solutions(soln: (String, String)) {
    println!("part 1: {}\npart 2: {}", soln.0, soln.1)
}

fn run_day1() -> (String, String) {
    let input = load_input(1);
    
    let mut vec = vec![0];
    for line in input.lines() {
        if line.len() == 0 {
            vec.push(0);
            continue;
        }
        
        let v: i32 = line.parse().unwrap();
        let current = vec.last_mut().unwrap();
        *current += v;
    }

    vec.sort();
    vec.reverse();
    let part1 = vec.first().unwrap();
    let part2: i32 = vec.iter().take(3).sum();

    return (part1.to_string(), part2.to_string());
}

fn load_input(day: i32) -> String {
    let f = format!("./input/day_{day}.txt");
    let input = fs::read_to_string(Path::new(&f)).
        expect("Input should exist");

    return input;
}
