use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Tree,
}
impl Cell {
    fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }
}
impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Tree,
            _ => Cell::Empty,
        }
    }
}

fn part1(grid: &Grid<Cell>) -> usize {
    test_slope(grid, 3, 1)
}

fn part2(grid: &Grid<Cell>) -> usize {
    let mut p = test_slope(grid, 1, 1);
    p *= test_slope(grid, 3, 1);
    p *= test_slope(grid, 5, 1);
    p *= test_slope(grid, 7, 1);
    p *= test_slope(grid, 1, 2);
    p
}

fn test_slope(grid: &Grid<Cell>, xs: i64, ys: i64) -> usize {
    let mut x = 0i64;
    let mut y = 0i64;
    let width = grid.x_bounds().end;
    let max_y = grid.y_bounds().end;
    let mut count = 0;

    while y < max_y {
        if !grid.get(x % width, y).is_empty() {
            count += 1;
        }
        x += xs;
        y += ys;
    }
    count
}

fn main() {
    let input:Vec<String> = read_input();
    let grid = Grid::from_input(&input, Cell::Empty, 0);
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day03_test() {
        let input:Vec<String> = test_input(include_str!("day03.testinput"));
        let grid = Grid::from_input(&input, Cell::Empty, 0);
        assert_eq!(part1(&grid), 7);
        assert_eq!(part2(&grid), 336);
    }
}
