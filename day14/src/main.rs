fn main() {
    let input = "084601";

    let offset: usize = input.parse().unwrap();
    let search: Vec<u8> = input.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();

    let mut scoreboard: Vec<u8> = vec![3, 7];

    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        let rcp1 = scoreboard[elf1];
        let rcp2 = scoreboard[elf2];
        let next = rcp1 + rcp2;

        if next >= 10 {
            scoreboard.push(next / 10);
            if scoreboard.ends_with(&search) {
                break;
            }
        }

        scoreboard.push(next % 10);

        if scoreboard.ends_with(&search) {
            break;
        }

        elf1 = (elf1 + rcp1 as usize + 1) % scoreboard.len();
        elf2 = (elf2 + rcp2 as usize + 1) % scoreboard.len();
    }

    let part1: String = scoreboard[offset..offset + 10]
        .iter()
        .map(|e| e.to_string())
        .collect();

    let part2 = scoreboard.len() - search.len();

    println!("{} {}", part1, part2);
}
