

// There's a map of nearby hiking trails (your puzzle input) 
// that indicates paths (.), forest (#), and steep slopes (^, >, v, and <).

use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[test]
fn test() {
    let input = {"
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
".trim()};

    let input = include_str!("input");

    let map = input.split("\n")
        .map(|s|s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let map_size = map.len();
    assert_eq!(map_size, map[0].len());

    let start = Pos::new(0, 1);
    let end = Pos::new(map_size - 1, map_size - 2);

    assert_eq!(map[start.row][start.col], '.');
    assert_eq!(map[end.row][end.col], '.');

    let mut path = HashSet::new();
    path.insert(start);
    let steps = dfs(&map, path, start, end);
    println!("steps: {:?}", steps-1);

    // Part Two
    let mut dead_ends = HashSet::new();
    let mut path = HashSet::new();
    path.insert(start);
    let steps = dfs_part_2(&map, path, None, start, end, &mut dead_ends);
    println!("steps: {:?}", steps-1);
}

fn dfs(map: &Vec<Vec<char>>, path: HashSet<Pos>, start: Pos, end: Pos) -> usize {
    if start == end {
        return path.len();
    }

    let mut path = path;
    let map_size = map.len();
    let mut start = start;

    let mut new_positions = Vec::new();

    loop {
        let Pos { row, col } = start;

        new_positions.clear();

        {
            // up
            if row > 0 {
                let row = row - 1;
                if map[row][col] != 'v' {
                    new_positions.push(Pos::new(row, col));
                }
            }
            // down
            if row < map_size - 1 {
                let row = row + 1;
                if map[row][col] != '^' {
                    new_positions.push(Pos::new(row, col));
                }
            }
            // left
            if col > 0 {
                let col = col - 1;
                if map[row][col] != '>' {
                    new_positions.push(Pos::new(row, col));
                }
            }
            // right
            if col < map_size - 1 {
                let col = col + 1;
                if map[row][col] != '<' {
                    new_positions.push(Pos::new(row, col));
                }
            }
        }

        new_positions = new_positions.into_iter()
            .filter(|pos| !path.contains(pos))
            .filter(|pos| map[pos.row][pos.col] != '#')
            .collect();

        if new_positions.len() == 0 {
            return path.len();
        } else if new_positions.len() == 1 {
            let pos = new_positions[0];
            path.insert(pos);
            start = pos;
        } else {
            break;
        }
    }

    {
        new_positions.into_iter()
            .map(|pos| {
                // let ch = map[pos.row][pos.col];
                let mut path = path.clone();
                path.insert(pos);
                dfs(map, path, pos, end)
            })
            .max()
            .unwrap()
    }
}

fn dfs_part_2(map: &Vec<Vec<char>>, path: HashSet<Pos>, mut pre_pos: Option<Pos>, start: Pos, end: Pos, dead_ends: &mut HashSet<Pos>) -> usize {
    if start == end {
        return path.len();
    }

    let this_start = start;

    let mut path = path;
    let map_size = map.len();
    let mut start = start;

    let mut new_positions = Vec::new();

    loop {
        let Pos { row, col } = start;

        new_positions.clear();

        {
            // up
            if row > 0 {
                let row = row - 1;
                let pos = Pos::new(row, col);
                if !matches!(pre_pos, Some(p) if p==pos) && map[row][col] != '#' {new_positions.push(Pos::new(row, col));}
            }
            // down
            if row < map_size - 1 {
                let row = row + 1;
                let pos = Pos::new(row, col);
                if !matches!(pre_pos, Some(p) if p==pos) && map[row][col] != '#' {new_positions.push(Pos::new(row, col));}
            }
            // left
            if col > 0 {
                let col = col - 1;
                let pos = Pos::new(row, col);
                if !matches!(pre_pos, Some(p) if p==pos) && map[row][col] != '#' {new_positions.push(Pos::new(row, col));}
            }
            // right
            if col < map_size - 1 {
                let col = col + 1;
                let pos = Pos::new(row, col);
                if !matches!(pre_pos, Some(p) if p==pos) && map[row][col] != '#' {new_positions.push(Pos::new(row, col));}
            }
        }

        // dead end
        if new_positions.is_empty() {
            return 0;
        }

        new_positions = new_positions.into_iter()
            .filter(|pos| !path.contains(pos))
            .filter(|pos| !dead_ends.contains(pos))
            .filter(|pos| map[pos.row][pos.col] != '#')
            .collect();

        if new_positions.len() == 0 {
            return 0;
        } else if new_positions.len() == 1 {
            let pos = new_positions[0];
            path.insert(pos);
            pre_pos = Some(start);
            start = pos;

            // println!("{:?}", pos);

            if start == end {
                return path.len();
            }            
        } else {
            break;
        }
    }

    {
        new_positions.into_iter()
            .map(|pos| {
                let mut path = path.clone();
                path.insert(pos);
                let len = dfs_part_2(map, path, Some(start), pos, end, dead_ends);

                // dead end
                if len == 0 {
                    dead_ends.insert(pos);
                    0
                } else {
                    len
                }
            })
            .max()
            .unwrap()
    }
}