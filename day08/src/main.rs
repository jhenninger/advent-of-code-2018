use std::str::FromStr;

struct Node {
    metadata: Vec<usize>,
    children: Vec<Node>,
}

impl Node {
    fn from_iter(iter: &mut Iterator<Item = usize>) -> Option<Node> {
        let child_count = iter.next()?;
        let meta_count = iter.next()?;

        let children = (0..child_count)
            .map(|_| Node::from_iter(iter))
            .collect::<Option<_>>()?;

        let metadata: Vec<_> = iter.take(meta_count).collect();

        if metadata.len() == meta_count {
            Some(Node { children, metadata })
        } else {
            None
        }
    }

    fn part_1(&self) -> usize {
        self.metadata.iter().sum::<usize>() + self.children.iter().map(Node::part_1).sum::<usize>()
    }

    fn part_2(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.children
                .iter()
                .enumerate()
                .map(
                    |(i, c)| match self.metadata.iter().filter(|m| **m == i + 1).count() {
                        0 => 0,
                        count => count * c.part_2(),
                    },
                )
                .sum()
        }
    }
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .split_whitespace()
            .map(|s| s.parse())
            .take_while(Result::is_ok)
            .flatten();
        Node::from_iter(&mut iter).ok_or("parse error")
    }
}

fn main() {
    let input = include_str!("../input").trim();
    let root: Node = input.parse().unwrap();

    println!("{}", root.part_1());
    println!("{}", root.part_2());
}
