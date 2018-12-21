use std::collections::HashSet;

fn main() {
    let input = include_str!("../input").trim();

    let mut stack = Vec::new();
    let mut path = String::new();
    let mut paths = HashSet::new();

    for c in input.chars() {
        match c {
            '^' | '$' => (),
            'N' | 'S' | 'E' | 'W' => {
                if path.ends_with(opposite(c)) {
                    path.pop();
                } else {
                    path.push(c);
                    paths.insert(path.clone());
                }
            }
            '(' => stack.push(path.len()),
            ')' => path.truncate(stack.pop().expect("parentheses mismatch")),
            '|' => path.truncate(*stack.last().expect("alternative not in parentheses")),
            invalid => panic!("invalid char '{}'", invalid),
        }
    }

    println!("part 1: {}", paths.iter().map(|s| s.len()).max().unwrap_or(0));
    println!("part 2: {}", paths.iter().filter(|s| s.len() >= 1000).count());
}

fn opposite(c: char) -> char {
    match c {
        'N' => 'S',
        'S' => 'N',
        'E' => 'W',
        'W' => 'E',
        _ => panic!(),
    }
}
