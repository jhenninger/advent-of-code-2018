fn main() {
    let input = include_str!("../input").trim();

    let input: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", part_1(&input).1);
    println!("{}", part_2(&input).1);
}

fn part_1(input: &[usize]) -> (usize, usize) {
    let child_count = input[0];
    let meta_count = input[1];

    let mut meta = 0;
    let mut len = 2;

    for _ in 0..child_count {
        let (child_len, child_meta) = part_1(&input[len..]);
        len += child_len;
        meta += child_meta;
    }

    meta += input[len..len + meta_count].iter().sum::<usize>();

    (len + meta_count, meta)
}

fn part_2(input: &[usize]) -> (usize, usize) {
    let child_count = input[0];
    let meta_count = input[1];

    let mut meta = 0;
    let mut len = 2;

    if child_count > 0 {
        let mut children = Vec::with_capacity(child_count);

        for _ in 0..child_count {
            let (child_len, child_meta) = part_2(&input[len..]);
            len += child_len;
            children.push(child_meta);
        }

        meta += input[len..len + meta_count]
            .iter()
            .filter(|&idx| *idx > 0)
            .map(|&idx| children.get(idx - 1).unwrap_or(&0))
            .sum::<usize>();
    } else {
        meta += input[len..len + meta_count].iter().sum::<usize>();
    }

    (len + meta_count, meta)
}
