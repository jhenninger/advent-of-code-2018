use regex::Regex;
use std::collections::HashMap;

struct Guard([u32; 60]);

impl Guard {
    fn add_sleep(&mut self, from: usize, to: usize) {
        for i in from..to {
            self.0[i] += 1;
        }
    }

    fn total_sleep(&self) -> u32 {
        self.0.iter().sum()
    }

    fn sleepiest_minute(&self) -> Option<(usize, u32)> {
        self.0.iter().cloned().enumerate().max_by_key(|p| p.1)
    }
}

impl Default for Guard {
    fn default() -> Self {
        Guard([0; 60])
    }
}

fn main() {
    let input = include_str!("../input");

    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort_unstable();

    let r_m = Regex::new(r":(\d\d)]").unwrap();
    let r_id = Regex::new(r"#(\d+)").unwrap();

    let mut guards: HashMap<u32, Guard> = HashMap::new();
    let mut guard = None;
    let mut from = None;

    for line in lines {
        if line.contains("Guard") {
            let id: u32 = r_id.captures(line).unwrap()[1].parse().unwrap();
            guard = Some(guards.entry(id).or_default());
            continue;
        }

        let guard = guard.as_mut().unwrap();
        let minute = r_m.captures(line).unwrap()[1].parse().unwrap();

        if line.contains("asleep") {
            from = Some(minute);
        } else {
            guard.add_sleep(from.unwrap(), minute);
        }
    }

    let part_1 = guards
        .iter()
        .max_by_key(|(_, g)| g.total_sleep())
        .and_then(|(id, g)| g.sleepiest_minute().map(|(m, _)| m as u32 * id));

    println!("{:?}", part_1);

    let part_2 = guards
        .iter()
        .flat_map(|(id, g)| g.sleepiest_minute().map(|(m, t)| (id, m, t)))
        .max_by_key(|(_, _, t)| *t)
        .map(|(id, m, _)| id * m as u32);

    println!("{:?}", part_2);
}
