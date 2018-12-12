struct Grid([[i64; 300]; 300]);

impl Grid {
    fn new(serial: i64) -> Self {
        let mut grid = [[0; 300]; 300];

        for (y, row) in grid.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                *cell = Self::power_level(x as i64, y as i64, serial);
            }
        }

        Grid(grid)
    }

    fn power_level(x: i64, y: i64, serial: i64) -> i64 {
        let rack_id = x + 10;
        (rack_id * y + serial) * rack_id / 100 % 10 - 5
    }

    fn max_square(&self, size: usize) -> (usize, usize, i64) {
        let upper = 300 - size;

        (0..=upper)
            .flat_map(|x| (0..=upper).map(move |y| (x, y)))
            .map(|(x, y)| (x, y, self.square_power(x, y, size)))
            .max_by_key(|t| t.2)
            .unwrap()
    }

    fn square_power(&self, x: usize, y: usize, size: usize) -> i64 {
        (x..x + size)
            .flat_map(|x| (y..y + size).map(move |y| self.0[y][x]))
            .sum()
    }

    fn max(&self) -> (usize, usize, i64, usize) {
        (1..=300)
            .map(|size| {
                let (x, y, p) = self.max_square(size);
                (x, y, p, size)
            })
            .max_by_key(|t| t.2)
            .unwrap()
    }
}

fn main() {
    let serial = 9445;

    let grid = Grid::new(serial);
    let (x, y, _) = grid.max_square(3);
    println!("{},{}", x, y);
    let (x, y, _, s) = grid.max();
    println!("{},{},{}", x, y, s);
}
