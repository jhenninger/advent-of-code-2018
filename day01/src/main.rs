use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    let frequencies: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();

    println!("{}", frequencies.iter().sum::<i32>());

    let mut seen = HashSet::new();
    let mut current = 0;

    for n in frequencies.iter().cycle() {
        current += n;
        if !seen.insert(current) {
            println!("{}", current);
            break;
        }
    }
}
