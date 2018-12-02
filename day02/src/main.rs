use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");
    let mut twos = 0;
    let mut threes = 0;

    for line in input.lines() {
        let mut map = HashMap::new();
        for c in line.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        if map.values().any(|&v| v == 2) {
            twos += 1;
        }

        if map.values().any(|&v| v == 3) {
            threes += 1;
        }
    }

    println!("{}", twos * threes);

    'outer: for (i, w1) in input.lines().enumerate() {
        for w2 in input.lines().skip(i + 1) {
            let diffs = w1.chars().zip(w2.chars())
                .filter(|(a, b)| a != b)
                .count();

            if diffs == 1 {
                let part_2: String = w1.chars().zip(w2.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect();

                println!("{}", part_2);
                break 'outer;
            }
        }
    }
}
