use std::collections::{HashMap,HashSet,VecDeque};
use std::vec::Vec;
use ya_advent_lib::read::read_grouped_input;
use ya_advent_lib::grid::Grid;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Edge {
    Top,
    TopRev,
    Bot,
    BotRev,
    Left,
    LeftRev,
    Right,
    RightRev,
}

#[derive(Debug, Clone, Copy)]
enum Orient {
    Orig,
    Rot90,
    Rot180,
    Rot270,
    Flip,
    Rot90Flip,
    Rot180Flip,
    Rot270Flip
}

impl Orient {
    fn map(&self, x: i64, y: i64, w: i64, h: i64) -> (i64, i64) {
        match self {
            Orient::Orig => (x, y),
            Orient::Flip => (w-1-x, y),
            Orient::Rot90 => (h-1-y, x),
            Orient::Rot180 => (w-1-x, h-1-y),
            Orient::Rot270 => (y, w-1-x),
            Orient::Rot90Flip => (y, x),
            Orient::Rot180Flip => (x, h-1-y),
            Orient::Rot270Flip => (h-1-y, w-1-x),
        }
    }
}

struct Tile {
    id: u64,
    grid: Grid<bool>,
    edge_set: HashMap<String, Edge>,
    edge: HashMap<Edge, String>,
}

impl Tile {
    fn new(input: &[String]) -> Self {
        let id = input[0]
            .split(' ').nth(1).unwrap()
            .split(':').next().unwrap()
            .parse::<u64>().unwrap();
        let data:Vec<String> = input.iter().skip(1).cloned().collect();

        let grid = Grid::from_input_map(&data, false, 0,
            |c| match c { '#' => true, '.' => false, _ => panic!() });

        let edge = Self::extract_edges(&grid);

        Self {
            id,
            grid,
            edge_set: HashMap::from_iter(edge.iter().map(|(k,v)| (v.clone(), *k))),
            edge,
        }
    }

    fn extract_edges(grid: &Grid<bool>) -> HashMap<Edge,String> {
        let end = grid.x_bounds().end - 1;
        let top = grid.x_bounds().map(|x| if grid.get(x, 0) {'#'} else {'.'}).collect::<String>();
        let bot = grid.x_bounds().map(|x| if grid.get(x, end) {'#'} else {'.'}).collect::<String>();
        let right = grid.y_bounds().map(|y| if grid.get(end, y) {'#'} else {'.'}).collect::<String>();
        let left = grid.y_bounds().map(|y| if grid.get(0, y) {'#'} else {'.'}).collect::<String>();
        let mut edge = HashMap::with_capacity(8);
        edge.insert(Edge::TopRev, top.chars().rev().collect());
        edge.insert(Edge::Top, top);
        edge.insert(Edge::BotRev, bot.chars().rev().collect());
        edge.insert(Edge::Bot, bot);
        edge.insert(Edge::RightRev, right.chars().rev().collect());
        edge.insert(Edge::Right, right);
        edge.insert(Edge::LeftRev, left.chars().rev().collect());
        edge.insert(Edge::Left, left);
        edge
    }

    fn rotate(&self, orient: Orient) -> Self {
        let mut grid = self.grid.clone_without_data(false);
        let m = grid.x_bounds().end;
        assert_eq!(grid.x_bounds(), grid.y_bounds());

        for y in grid.y_bounds() {
            for x in grid.x_bounds() {
                let (newx, newy) = orient.map(x, y, m, m);
                grid.set(newx, newy, self.grid.get(x, y))
            }
        }

        let edge = Self::extract_edges(&grid);

        Self {
            id: self.id,
            grid,
            edge_set: HashMap::from_iter(edge.iter().map(|(k,v)| (v.clone(), *k))),
            edge,
        }
    }
}

fn mktiles(input: Vec<Vec<String>>) -> Vec<Tile> {
    input
        .iter()
        .map(|i| Tile::new(i))
        .collect()
}

fn part1(tiles: &[Tile]) -> u64 {
    let mut matches: HashMap<u64, usize> = HashMap::new();
    for a in 0 .. tiles.len() {
        for b in a+1 .. tiles.len() {
            if tiles[a].edge_set
                .iter()
                .any(|(k,_)| tiles[b].edge_set.contains_key(k)) {
                    matches.entry(tiles[a].id)
                        .and_modify(|m| *m += 1)
                        .or_insert(1);
                    matches.entry(tiles[b].id)
                        .and_modify(|m| *m += 1)
                        .or_insert(1);
            }
        }
    }
    matches.iter().filter(|(_,v)| **v == 2).map(|(k,_)| k).product()
}

struct TilePos {
    x: i64,
    y: i64,
    tile: Tile,
}

fn arrange(tiles: &[Tile]) -> Grid<bool> {
    let mut found: HashMap<u64, TilePos> = HashMap::with_capacity(tiles.len());
    found.insert(tiles[0].id, TilePos { x:0, y:0, tile: tiles[0].rotate(Orient::Orig) });
    let mut queue: VecDeque<u64> = VecDeque::new();
    queue.push_back(tiles[0].id);
    while let Some(tile_id) = queue.pop_front() {
        for edge in [ Edge::Top, Edge::Bot, Edge::Left, Edge::Right ] {
            if let Some((other_t, other_e)) = tiles.iter()
                .filter(|&t| !found.contains_key(&t.id))
                .find(|&t| t.edge_set.contains_key(&found[&tile_id].tile.edge[&edge]))
                .map(|t| (t, t.edge_set[&found[&tile_id].tile.edge[&edge]]))
            {
                let (orient, x_off, y_off) = match (edge, other_e) {
                    (Edge::Top, Edge::Top) => (Orient::Rot180Flip, 0, -1),
                    (Edge::Top, Edge::TopRev) => (Orient::Rot180, 0, -1),
                    (Edge::Top, Edge::Bot) => (Orient::Orig, 0, -1),
                    (Edge::Top, Edge::BotRev) => (Orient::Flip, 0, -1),
                    (Edge::Top, Edge::Right) => (Orient::Rot90Flip, 0, -1),
                    (Edge::Top, Edge::RightRev) => (Orient::Rot90, 0, -1),
                    (Edge::Top, Edge::Left) => (Orient::Rot270, 0, -1),
                    (Edge::Top, Edge::LeftRev) => (Orient::Rot270Flip, 0, -1),

                    (Edge::Left, Edge::Top) => (Orient::Rot90, -1, 0),
                    (Edge::Left, Edge::TopRev) => (Orient::Rot270Flip, -1, 0),
                    (Edge::Left, Edge::Bot) => (Orient::Rot90Flip, -1, 0),
                    (Edge::Left, Edge::BotRev) => (Orient::Rot270, -1, 0),
                    (Edge::Left, Edge::Right) => (Orient::Orig, -1, 0),
                    (Edge::Left, Edge::RightRev) => (Orient::Rot180Flip, -1, 0),
                    (Edge::Left, Edge::Left) => (Orient::Flip, -1, 0),
                    (Edge::Left, Edge::LeftRev) => (Orient::Rot180, -1, 0),

                    (Edge::Right, Edge::Top) => (Orient::Rot90Flip, 1, 0),
                    (Edge::Right, Edge::TopRev) => (Orient::Rot270, 1, 0),
                    (Edge::Right, Edge::Bot) => (Orient::Rot90, 1, 0),
                    (Edge::Right, Edge::BotRev) => (Orient::Rot270Flip, 1, 0),
                    (Edge::Right, Edge::Right) => (Orient::Flip, 1, 0),
                    (Edge::Right, Edge::RightRev) => (Orient::Rot180, 1, 0),
                    (Edge::Right, Edge::Left) => (Orient::Orig, 1, 0),
                    (Edge::Right, Edge::LeftRev) => (Orient::Rot180Flip, 1, 0),

                    (Edge::Bot, Edge::Top) => (Orient::Orig, 0, 1),
                    (Edge::Bot, Edge::TopRev) => (Orient::Flip, 0, 1),
                    (Edge::Bot, Edge::Bot) => (Orient::Rot180Flip, 0, 1),
                    (Edge::Bot, Edge::BotRev) => (Orient::Rot180, 0, 1),
                    (Edge::Bot, Edge::Right) => (Orient::Rot270, 0, 1),
                    (Edge::Bot, Edge::RightRev) => (Orient::Rot270Flip, 0, 1),
                    (Edge::Bot, Edge::Left) => (Orient::Rot90Flip, 0, 1),
                    (Edge::Bot, Edge::LeftRev) => (Orient::Rot90, 0, 1),

                    _ => panic!(),
                };
                let newtile = other_t.rotate(orient);
                let other_edge = match edge {
                    Edge::Top => Edge::Bot,
                    Edge::Left => Edge::Right,
                    Edge::Right => Edge::Left,
                    Edge::Bot => Edge::Top,
                    _ => panic!(),
                };
                assert_eq!(newtile.edge[&other_edge], found[&tile_id].tile.edge[&edge]);
                let tilepos = &found[&tile_id];
                let newtp = TilePos {
                    x: tilepos.x + x_off,
                    y: tilepos.y + y_off,
                    tile: newtile,
                };
                found.insert(other_t.id, newtp);
                queue.push_back(other_t.id);
            }
        }
    }
    assert!(found.len() == tiles.len());
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    found
        .iter()
        .for_each(|(_, tp)| {
            min_x = min_x.min(tp.x);
            max_x = max_x.max(tp.x);
            min_y = min_y.min(tp.y);
            max_y = max_y.max(tp.y);
        });

    let tile_size = tiles[0].edge[&Edge::Top].len() as i64 - 2;
    let mut grid = Grid::new(
        min_x * tile_size, min_y * tile_size,
        (max_x + 1) * tile_size - 1, (max_y + 1) * tile_size - 1,
        false
    );

    for (_, tp) in found {
        let xoff = tp.x * tile_size;
        let yoff = tp.y * tile_size;
        tp.tile.grid.for_each(|val, x, y| {
            if x > 0 && y > 0 && x <= tile_size && y <= tile_size {
                grid.set(xoff + x - 1, yoff + y - 1, val);
            }
        });
    }
    grid
}

struct Monster {
    coords: Vec<(i64, i64)>,
    width: i64,
    height: i64,
}
//           1111111111
// 01234567890123456789
// ..................#.
// #....##....##....###
// .#..#..#..#..#..#...

impl Monster {
    fn new(orient: Orient) -> Self {
        let coords: Vec<(i64, i64)> = vec![ (18, 0),
          (0, 1), (5, 1), (6, 1), (11, 1), (12, 1), (17, 1), (18, 1), (19, 1),
          (1, 2), (4, 2), (7, 2), (10, 2), (13, 2), (16, 2)
        ];
        let r90 = matches!(orient, Orient::Rot90 | Orient::Rot270 | Orient::Rot90Flip | Orient::Rot270Flip);
        Self {
            coords: coords.iter().map(|(x, y)| orient.map(*x, *y, 20, 3)).collect(),
            width: if r90 { 3 } else { 20 },
            height: if r90 { 20 } else { 3 },
        }
    }
    fn find_in_grid(&self, grid: &Grid<bool>) -> Vec<(i64,i64)> {
        let x_range = grid.x_bounds();
        let y_range = grid.y_bounds();
        let mut out = Vec::new();
        for y in y_range.start .. y_range.end - self.height {
            for x in x_range.start .. x_range.end - self.width {
                if self.coords
                    .iter()
                    .all(|(cx, cy)| grid.get(x + cx, y + cy)) {
                        out.push((x, y));
                }
            }
        }
        out
    }
}

fn part2(tiles: &[Tile]) -> usize {
    let grid = arrange(tiles);
    //dump_grid(&grid);
    for o in [Orient::Orig, Orient::Rot90, Orient::Rot180, Orient::Rot270,
              Orient::Flip, Orient::Rot90Flip, Orient::Rot180Flip, Orient::Rot270Flip] {
        let m = Monster::new(o);
        let matches = m.find_in_grid(&grid);
        if !matches.is_empty() {
            let monsterlocs:HashSet<(i64,i64)> = matches.iter()
                .flat_map(|(mx, my)| m.coords.iter().map(move |(cx, cy)| (*mx+*cx, *my+*cy)))
                .collect();
            return grid.iter_with_coord()
                .filter(|(v, x, y)| *v && !monsterlocs.contains(&(*x, *y)))
                .count();
        }
    }
    panic!();
}

#[allow(dead_code)]
fn printgrid(grid: &Grid<bool>) {
    grid.print_str(|c| if c {"#".into()} else {".".into()});
}
#[allow(dead_code)]
fn dump_grid(grid: &Grid<bool>) {
    let f = std::fs::File::create("day20-grid").unwrap();
    let mut stream = std::io::BufWriter::new(f);
    grid.dump_to_file(&mut stream, |c| if c {'#'} else {'.'});
}

fn main() {
    let input: Vec<Vec<String>> = read_grouped_input::<String>();
    let tiles = mktiles(input);
    println!("Part 1: {}", part1(&tiles));
    println!("Part 2: {}", part2(&tiles));
/*
    let input = vec![
        "Tile 0:".into(),
        "..........".into(),
        "..#####...".into(),
        "..#....#..".into(),
        "..#....#..".into(),
        "..#...#...".into(),
        "..####....".into(),
        "..#...#...".into(),
        "..#....#..".into(),
        "..#.....#.".into(),
        "..........".into(),
    ];
    let tile = Tile::new(&input);
    for o in [ Orient::Orig, Orient::Rot90, Orient::Rot180, Orient::Rot270,
        Orient::Flip, Orient::Rot90Flip, Orient::Rot180Flip, Orient::Rot270Flip ] {

        let rtile = tile.rotate(o);
        println!("{o:?}:");
        printgrid(&rtile.grid);
        println!("");
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::grouped_test_input;

    #[test]
    fn day20_test() {
        let input:Vec<Vec<String>> = grouped_test_input(include_str!("day20.testinput"));
        let tiles = mktiles(input);
        assert_eq!(part1(&tiles), 20899048083289);
        assert_eq!(part2(&tiles), 273);
    }
}
