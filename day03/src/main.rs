use regex::Regex;
use std::collections::HashSet;

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    h: u32,
    w: u32,
}

#[derive(Clone, PartialEq)]
enum Point {
    Free,
    Claimed(u32),
    Overlap,
}

use self::Point::*;

fn main() {
    let input = include_str!("../input");
    let regex = Regex::new(r"(?m)^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let claims: Vec<Claim> = regex
        .captures_iter(input)
        .map(|c| Claim {
            id: c[1].parse().unwrap(),
            x: c[2].parse().unwrap(),
            y: c[3].parse().unwrap(),
            w: c[4].parse().unwrap(),
            h: c[5].parse().unwrap(),
        })
        .collect();

    let mut fabric = vec![vec![Point::Free; 1000]; 1000];
    let mut overlap_ids = HashSet::new();

    for claim in &claims {
        for x in claim.x..(claim.x + claim.w) {
            for y in claim.y..(claim.y + claim.h) {

                let square = &mut fabric[y as usize][x as usize];

                *square = match square {
                    Free => Claimed(claim.id),
                    Claimed(id) => {
                        overlap_ids.insert(*id);
                        overlap_ids.insert(claim.id);
                        Overlap
                    }
                    Overlap => {
                        overlap_ids.insert(claim.id);
                        Overlap
                    },
                };
            }
        }
    }

    let overlaps = fabric
        .iter()
        .flatten()
        .filter(|point| **point == Overlap)
        .count();

    let claim = claims
        .iter()
        .find(|claim| !overlap_ids.contains(&claim.id))
        .unwrap();

    println!("{}\n{}", overlaps, claim.id);
}
