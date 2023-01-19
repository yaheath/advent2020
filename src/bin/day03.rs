extern crate advent2020;
use std::cmp::max;
use advent2020::read::read_input;
use advent2020::grid::Grid;

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Tree,
}
impl Cell {
    fn is_empty(&self) -> bool {
        match *self {
            Cell::Empty => true,
            _ => false,
        }
    }
}

fn main() {
    let data = read_input::<String>();
    let width = data.iter().map(|s| s.len()).fold(0, |maxw, w| max(w, maxw)) as i32;
    let height = data.len() as i32;
    let mut grid = Grid::new(0, 0, width-1, height-1, Cell::Empty);

    let mut y = 0i32;
    for line in data.iter() {
        for (ux, c) in line.chars().enumerate() {
            let x = ux as i32;
            let cell = match c {
                '#' => Cell::Tree,
                _ => Cell::Empty,
            };
            if !cell.is_empty() {
                grid.set(x, y, cell);
            }
        }
        y += 1;
    }

    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Grid<Cell>) {
    let count = test_slope(grid, 3, 1);
    println!("Part 1: {}", count);
}

fn part2(grid: &Grid<Cell>) {
    let mut p = test_slope(grid, 1, 1);
    p *= test_slope(grid, 3, 1);
    p *= test_slope(grid, 5, 1);
    p *= test_slope(grid, 7, 1);
    p *= test_slope(grid, 1, 2);
    println!("Part 2: {}", p);
}

fn test_slope(grid: &Grid<Cell>, xs: i32, ys: i32) -> i64 {
    let mut x = 0i32;
    let mut y = 0i32;
    let width = grid.x_bounds().end;
    let max_y = grid.y_bounds().end;
    let mut count = 0i64;

    while y < max_y {
        if !grid.get(x % width, y).is_empty() {
            count += 1;
        }
        x += xs;
        y += ys;
    }
    count
}
