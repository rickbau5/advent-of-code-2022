pub fn window_input(input: String, window_size: i32) -> Vec<Vec<String>> {
    let mut windows = vec![];
    let mut window = vec![];
    let mut index = 0;
    for line in input.lines() {
        if index == window_size {
            index = 0;
            windows.push(window);
            window = vec![];
        }
        window.push(line.to_string());
        index += 1;
    }
    windows.push(window.clone());
    window.clear();

    return windows;
}
