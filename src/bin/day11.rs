use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;

#[derive(Clone, Copy)]
enum Seat {
    Floor,
    Empty,
    Occupied,
    NextEmpty,
    NextOccupied,
}

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _ => Seat::Floor,
        }
    }
}

fn mkgrid(input: &[String]) -> Grid<Seat> {
    Grid::from_input(&input, Seat::Floor, 1)
}

fn neighbors_immed(x: i64, y: i64, grid: &Grid<Seat>) -> usize {
    [ (x-1, y-1), (x, y-1), (x+1, y-1),
      (x-1, y),             (x+1, y),
      (x-1, y+1), (x, y+1), (x+1, y+1) ]
        .iter()
        .map(|(x,y)| grid.get(*x, *y))
        .filter(|s| match s {
            Seat::Occupied | Seat::NextEmpty => true,
            _ => false,
        })
        .count()
}

fn neighbors_los(x: i64, y: i64, grid: &Grid<Seat>) -> usize {
    [ (-1, -1), (0, -1), (1, -1),
      (-1, 0),           (1, 0),
      (-1, 1),  (0, 1),  (1, 1) ]
        .iter()
        .filter(|(dx, dy)| {
            let mut ret = false;
            for n in 1.. {
                if !grid.x_bounds().contains(&(x + dx*n)) ||
                    !grid.y_bounds().contains(&(y + dy*n)) {
                        break;
                }
                match grid.get(x + dx*n, y + dy*n) {
                    Seat::Occupied | Seat::NextEmpty => {
                        ret = true;
                        break;
                    },
                    Seat::Empty | Seat::NextOccupied => {
                        break;
                    }
                    Seat::Floor => {},
                };
            }
            ret
        })
        .count()
}

fn step(grid: &mut Grid<Seat>, part2: bool) -> bool {
    let x_bounds = grid.x_bounds();
    let x_bounds = x_bounds.start + 1 .. x_bounds.end - 1;
    let y_bounds = grid.y_bounds();
    let y_bounds = y_bounds.start + 1 .. y_bounds.end - 1;
    let mut changed = false;
    let thresh = if part2 { 5 } else { 4 };
    for y in y_bounds.clone() {
        for x in x_bounds.clone() {
            let is_occupied = match grid.get(x, y) {
                Seat::Floor => { continue; }
                Seat::Occupied => true,
                Seat::Empty => false,
                _ => panic!(),
            };
            let neighbors = if part2 {
                neighbors_los(x, y, grid)
            } else {
                neighbors_immed(x, y, grid)
            };
            if neighbors == 0 && !is_occupied {
                changed = true;
                grid.set(x, y, Seat::NextOccupied);
            } else if neighbors >= thresh && is_occupied {
                changed = true;
                grid.set(x, y, Seat::NextEmpty);
            }
        }
    }
    if changed {
        for y in y_bounds.clone() {
            for x in x_bounds.clone() {
                let next = match grid.get(x, y) {
                    Seat::NextOccupied => Seat::Occupied,
                    Seat::NextEmpty => Seat::Empty,
                    _ => { continue; },
                };
                grid.set(x, y, next);
            }
        }
    }

    changed
}

fn part1(input: &[String]) -> usize {
    let mut grid = mkgrid(input);

    while step(&mut grid, false) { }

    grid.iter().filter(|s| matches!(s, Seat::Occupied)).count()
}

fn part2(input: &[String]) -> usize {
    let mut grid = mkgrid(input);

    while step(&mut grid, true) { }

    grid.iter().filter(|s| matches!(s, Seat::Occupied)).count()
}

fn main() {
    let input: Vec<String> = read_input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_test() {
        let input: Vec<String> = vec![
            "L.LL.LL.LL".into(),
            "LLLLLLL.LL".into(),
            "L.L.L..L..".into(),
            "LLLL.LL.LL".into(),
            "L.LL.LL.LL".into(),
            "L.LLLLL.LL".into(),
            "..L.L.....".into(),
            "LLLLLLLLLL".into(),
            "L.LLLLLL.L".into(),
            "L.LLLLL.LL".into(),
        ];

        assert_eq!(part1(&input), 37);
        assert_eq!(part2(&input), 26);
    }
}
