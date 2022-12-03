use std::{fs, path::Path};

#[allow(dead_code)]
pub fn load_test() -> String {
    let input = fs::read_to_string(Path::new("./input/test.txt")).expect("Input should exist");

    return input;
}

#[allow(dead_code)]
pub fn load_input(day: i32) -> String {
    let f = format!("./input/day_{day}.txt");

    let input = fs::read_to_string(Path::new(&f)).expect("Input should exist");

    return input;
}
