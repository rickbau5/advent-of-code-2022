fn run() -> (String, String) {
    let input = input::load_input(1);
    
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