const DIFF: u8 = b'a' - b'A';

fn main() {
    let input = include_str!("../input").trim();

    println!("{}", react(input.as_bytes()));

    let shortest = (b'a'..b'z')
        .map(|c| {
            let polymer: Vec<u8> = input.bytes().filter(|&b| c != b && c - DIFF != b).collect();
            react(&polymer)
        })
        .min()
        .unwrap();

    println!("{}", shortest);
}

fn react(polymer: &[u8]) -> usize {
    let mut result = Vec::new();

    for &c in polymer {
        match result.last() {
            Some(&l) if reacts(c, l) => {
                result.pop();
            }
            _ => result.push(c),
        }
    }

    result.len()
}

fn reacts(a: u8, b: u8) -> bool {
    if a > b {
        a - b == DIFF
    } else {
        b - a == DIFF
    }
}
