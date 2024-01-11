use std::collections::HashMap;
use std::collections::hash_map::{Iter,IterMut};
use std::ops::Range;
use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;

#[derive(Clone)]
struct GridND<T: Copy> {
    dims: usize,
    default: T,
    data: HashMap<Vec<i64>,T>,
    ranges: Vec<Range<i64>>,
}

impl <T: Copy> GridND<T> {
    pub fn new(dims: usize, default_val: T) -> Self {
        let mut ranges = Vec::with_capacity(dims);
        for _ in 0..dims {
            ranges.push(Range { start: 0, end: 0 });
        }
        Self {
            dims,
            default: default_val,
            data: HashMap::new(),
            ranges,
        }
    }

    pub fn import_to_plane<F>(&mut self, x_dim: usize, y_dim: usize, input: &Vec<String>, mapfunc: F)
            where F: Fn(char, &Vec<i64>) -> Option<T> {
        let mut y = 0i64;
        for line in input.iter() {
            for (ux, c) in line.chars().enumerate() {
                let x = ux as i64;
                let mut coord = vec![0; self.dims];
                coord[x_dim] = x;
                coord[y_dim] = y;
                if let Some(val) = mapfunc(c, &coord) {
                    self.set(&coord, val);
                }
            }
            y += 1;
        }
    }

    pub fn get(&self, coord: &Vec<i64>) -> T {
        if let Some(cell) = self.data.get(coord) {
            *cell
        }
        else {
            self.default
        }
    }

    pub fn set(&mut self, coord: &Vec<i64>, val: T) {
        self.data.insert(coord.clone(), val);
        for d in 0..self.dims {
            if self.ranges[d].is_empty() {
                self.ranges[d].start = coord[d];
                self.ranges[d].end = coord[d] + 1;
            }
            else if coord[d] < self.ranges[d].start {
                self.ranges[d].start = coord[d];
            }
            else if coord[d] >= self.ranges[d].end {
                self.ranges[d].end = coord[d] + 1;
            }
        }
    }

    pub fn iter(&self) -> Iter<Vec<i64>, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Vec<i64>, T> {
        self.data.iter_mut()
    }

    pub fn neighbors<'a>(&'a self, center: &'a Vec<i64>) -> impl Iterator<Item=(T, Vec<i64>)> + '_ {
        let ranges = vec![-1 .. 2; self.dims];
        ranges
            .into_iter()
            .map(|r| r.into_iter())
            .multi_cartesian_product()
            .filter(|v| !v.iter().all(|&c| c == 0))
            .map(|v| {
                let mut newv = v.clone();
                for i in 0..newv.len() {
                    newv[i] += center[i];
                }
                newv
            })
            .map(|v| (self.get(&v), v))
    }
}

#[derive(Copy, Clone)]
enum Cell {
    Inactive,
    Active,
    NextInactive,
    NextActive,
}

fn mkgrid(input: &Vec<String>, dims: usize) -> GridND<Cell> {
    let mut grid = GridND::new(dims, Cell::Inactive);
    grid.import_to_plane(0, 1, input, |c,_| match c {
        '.' => None,
        '#' => Some(Cell::Active),
        _ => panic!(),
    });
    grid
}

fn step(grid: &mut GridND<Cell>) {
    for coord in grid.ranges.iter()
            .map(|r| ((r.start - 1) .. (r.end + 1)).into_iter())
            .multi_cartesian_product() {
        let n = grid.neighbors(&coord)
            .filter(|(c,_)| match c {
                Cell::Active | Cell::NextInactive => true,
                _ => false,
            })
            .count();
        match grid.get(&coord) {
            Cell::Inactive if n == 3 => {
               grid.set(&coord, Cell::NextActive);
            },
            Cell::Active if n != 2 && n != 3 => {
               grid.set(&coord, Cell::NextInactive);
            },
            _ => {},
        }
    }
    grid.iter_mut().for_each(|(_,cell)| *cell = match *cell {
        Cell::NextActive => Cell::Active,
        Cell::NextInactive => Cell::Inactive,
        v => v,
    });
}

fn part1(input: &Vec<String>) -> usize {
    let mut grid = mkgrid(input, 3);
    for _ in 0..6 {
        step(&mut grid);
    }
    grid.iter()
        .filter(|(_,cell)| matches!(cell, Cell::Active))
        .count()
}

fn part2(input: &Vec<String>) -> usize {
    let mut grid = mkgrid(input, 4);
    for _ in 0..6 {
        step(&mut grid);
    }
    grid.iter()
        .filter(|(_,cell)| matches!(cell, Cell::Active))
        .count()
}

fn main() {
    let input = read_input::<String>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17_test() {
        let input: Vec<String> = vec![
            ".#.".into(),
            "..#".into(),
            "###".into(),
        ];
        assert_eq!(part1(&input), 112);
        assert_eq!(part2(&input), 848);
    }
}
