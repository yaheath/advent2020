use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::infinite_grid::InfiniteGrid;

#[derive(Clone,Copy,Debug)]
enum Dir {
    NW,
    NE,
    E,
    SE,
    SW,
    W
}

#[derive(Clone)]
struct MoveSet(Vec<Dir>);

impl FromStr for MoveSet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
        let mut out: Vec<Dir> = Vec::new();
        while let Some(c) = iter.next() {
            match c {
                'e' => { out.push(Dir::E); },
                'w' => { out.push(Dir::W); },
                'n' => {
                    match iter.next().unwrap() {
                        'e' => { out.push(Dir::NE); },
                        'w' => { out.push(Dir::NW); },
                        _ => { return Err(()); },
                    }
                },
                's' => {
                    match iter.next().unwrap() {
                        'e' => { out.push(Dir::SE); },
                        'w' => { out.push(Dir::SW); },
                        _ => { return Err(()); },
                    }
                },
                _ => { return Err(()); },
            }
        }
        Ok(Self(out))
    }
}

#[derive(Copy, Clone)]
enum Tile {
    White,
    Black,
    NextWhite,
    NextBlack,
}

fn neighbor(x: i64, y: i64, dir: Dir) -> (i64, i64) {
    let mut x = x;
    let mut y = y;
    match dir {
        Dir::E => { x += 1; },
        Dir::W => { x -= 1; },
        Dir::NE if y & 1 == 0 => { y -= 1; },
        Dir::NE => { x += 1; y -= 1; },
        Dir::NW if y & 1 == 0 => { x -= 1; y -= 1; },
        Dir::NW => { y -= 1; },
        Dir::SE if y & 1 == 0 => { y += 1; },
        Dir::SE => { x += 1; y += 1; },
        Dir::SW if y & 1 == 0 => { x -= 1; y += 1; },
        Dir::SW => { y += 1; },
    }
    (x, y)
}

fn apply_moves(grid: &mut InfiniteGrid<Tile>, move_set: &MoveSet) {
    let mut x = 0;
    let mut y = 0;
    for dir in &move_set.0 {
        (x, y) = neighbor(x, y, *dir);
    }
    grid.set(x, y, match grid.get(x, y) {
        Tile::White => Tile::Black,
        Tile::Black => Tile::White,
        _ => panic!(),
    });
}

fn part1(input: &Vec<MoveSet>) -> usize {
    let mut grid: InfiniteGrid<Tile> = InfiniteGrid::new(Tile::White);
    for i in input {
        apply_moves(&mut grid, &i);
    }
    grid.iter().filter(|(_, &t)| matches!(t, Tile::Black)).count()
}

fn step(grid: &mut InfiniteGrid<Tile>) {
    let xb = grid.x_bounds();
    let yb = grid.y_bounds();
    for y in yb.start - 1 .. yb.end + 1 {
        for x in xb.start - 1 .. xb.end + 1 {
            let n = [ Dir::E, Dir::W, Dir::NE, Dir::NW, Dir::SE, Dir::SW ]
                .iter()
                .filter(|&&d| {
                    let (nx, ny) = neighbor(x, y, d);
                    match grid.get(nx, ny) {
                        Tile::Black | Tile::NextWhite => true,
                        _ => false
                    }
                })
                .count();
            let c = grid.get(x, y);
            match c {
                Tile::Black => {
                    if n == 0 || n > 2 {
                        grid.set(x, y, Tile::NextWhite);
                    }
                },
                Tile::White => {
                    if n == 2 {
                        grid.set(x, y, Tile::NextBlack);
                    }
                },
                _ => panic!(),
            }
        }
    }
    grid.iter_mut().for_each(|(_, t)| match *t {
        Tile::NextWhite => { *t = Tile::White; },
        Tile::NextBlack => { *t = Tile::Black; },
        _ => {},
    });
}

fn part2(input: &Vec<MoveSet>) -> usize {
    let mut grid: InfiniteGrid<Tile> = InfiniteGrid::new(Tile::White);
    for i in input {
        apply_moves(&mut grid, &i);
    }
    for _ in 0..100 {
        step(&mut grid);
    }
    grid.iter().filter(|(_, &t)| matches!(t, Tile::Black)).count()
}

fn main() {
    let input: Vec<MoveSet> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day24_test() {
        let input:Vec<MoveSet> = test_input(include_str!("day24.testinput"));
        assert_eq!(part1(&input), 10);
        assert_eq!(part2(&input), 2208);
    }
}
