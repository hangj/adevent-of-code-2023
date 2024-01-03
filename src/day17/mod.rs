use std::collections::BTreeSet;

fn calc_no_dir_len_limit(map: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let heigth = map.len();
    let width = map[0].len();

    let (dst_row, dst_col) = (heigth-1, width-1);

    let mut ret = Vec::with_capacity(heigth);
    ret.resize_with(heigth, || {
        let mut v = Vec::<usize>::with_capacity(width);
        v.resize(width, 0);
        v
    });

    ret[dst_row][dst_col] = 0;

    let mut starts = BTreeSet::new();

    // up
    if dst_row > 0 {
        let row = dst_row - 1;
        starts.insert((row, dst_col));
    }
    // left
    if dst_col > 0 {
        let col = dst_col - 1;
        starts.insert((dst_row, col));
    }

    loop {
        if starts.is_empty() { break; }

        let mut new_round = BTreeSet::new();

        for (row, col) in starts.iter() {
            let (row, col) = (*row, *col);
            if row > 0 { new_round.insert((row-1, col)); }
            if col > 0 { new_round.insert((row, col-1)); }

            ret[row][col] = 
                if (row + 1 == dst_row && col == dst_col) || (row == dst_row && col + 1 == dst_col) {
                    map[dst_row][dst_col]
                } else {
                    let mut min = usize::MAX;

                    if row < heigth - 1 {
                        min = min.min(ret[row + 1][col] + map[row + 1][col]);
                    }
                    if col < width - 1 {
                        min = min.min(ret[row][col + 1] + map[row][col + 1]);
                    }
                    min
                };
        }

        starts = new_round;
    }

    ret
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn calc(map: &Vec<Vec<usize>>) -> usize {
    let mut path_right = BTreeSet::new();
    path_right.insert(Pos::new(0, 1));
    let right = do_calc(map, path_right, Pos::new(0, 1), Dir::Right, 1).unwrap() + map[0][1];

    let mut path_down = BTreeSet::new();
    path_down.insert(Pos::new(1, 0));
    let down  = do_calc(map, path_down, Pos::new(1, 0), Dir::Down, 1).unwrap() + map[1][0];
    right.min(down)
}


fn do_calc(map: &Vec<Vec<usize>>, path: BTreeSet<Pos>, pos: Pos, dir: Dir, count: i32) -> Option<usize> {
    let heigth = map.len();
    let width = map[0].len();

    let (dst_row, dst_col) = (heigth-1, width-1);
    let Pos { row, col } = pos;

    if row == dst_row && col == dst_col {
        return Some(0);
    }

    let mut min = usize::MAX;

    match dir {
        Dir::Up => {
            // up
            if count < 3 && row > 0 {
                let count = count + 1;
                let row = row - 1;
                let pos = Pos::new(row, col);
                let mut path = path.clone();
                if path.insert(pos) {
                    if let Some(x) = do_calc(map, path, pos, dir, count) {
                        min = min.min(x + map[row][col]);
                    }
                }
            }
            // left
            if col > 0 {
                let count = 1;
                let dir = Dir::Left;
                let col = col - 1;
                let pos = Pos::new(row, col);
                let mut path = path.clone();
                if path.insert(pos) {
                    if let Some(x) = do_calc(map, path, pos, dir, count) {
                        min = min.min(x + map[row][col]);
                    }
                }
            }
            // right
            if col < width - 1 {
                let count = 1;
                let dir = Dir::Right;
                let col = col + 1;
                let pos = Pos::new(row, col);
                let mut path = path.clone();
                if path.insert(pos) {
                    if let Some(x) = do_calc(map, path, pos, dir, count) {
                        min = min.min(x + map[row][col]);
                    }
                }
            }
        },
        Dir::Down => {
            // down
            if count < 3 && row < heigth - 1 {
                let count = count + 1;
                let row = row + 1;
                let pos = Pos::new(row, col);
                let mut path = path.clone();
                if path.insert(pos) {
                    if let Some(x) = do_calc(map, path, pos, dir, count) {
                        min = min.min(x + map[row][col]);
                    }
                }
            }
            // left
            if col > 0 {
                let count = 1;
                let dir = Dir::Left;
                let col = col - 1;
                let pos = Pos::new(row, col);
                let mut path = path.clone();
                if path.insert(pos) {
                    if let Some(x) = do_calc(map, path, pos, dir, count) {
                        min = min.min(x + map[row][col]);
                    }
                }
            }
            // right
            if col < width - 1 {
                let count = 1;
                let dir = Dir::Right;
                let col = col + 1;
                let pos = Pos::new(row, col);
                let mut path = path.clone();
                if path.insert(pos) {
                    if let Some(x) = do_calc(map, path, pos, dir, count) {
                        min = min.min(x + map[row][col]);
                    }
                }
            }
        },
        Dir::Left => {
            // left
            if count < 3 && col > 0 {
                let count = count + 1;
                let col = col - 1;
                let pos = Pos::new(row, col);
                let mut path = path.clone();
                if path.insert(pos) {
                    if let Some(x) = do_calc(map, path, pos, dir, count) {
                        min = min.min(x + map[row][col]);
                    }
                }
            }
            // up
            if row > 0 {
                let row = row - 1;
                let dir = Dir::Up;
                let pos = Pos::new(row, col);
                let mut path = path.clone();
                if path.insert(pos) {
                    if let Some(x) = do_calc(map, path, pos, dir, count) {
                        min = min.min(x + map[row][col]);
                    }
                }
            }
            // down
            if row < heigth - 1 {
                let row = row + 1;
                let dir = Dir::Down;
                let pos = Pos::new(row, col);
                let mut path = path.clone();
                if path.insert(pos) {
                    if let Some(x) = do_calc(map, path, pos, dir, count) {
                        min = min.min(x + map[row][col]);
                    }
                }
            }

        },
        Dir::Right => {
            // right
            if count < 3 && col < width - 1 {
                let count = count + 1;
                let col = col + 1;
                let pos = Pos::new(row, col);
                let mut path = path.clone();
                if path.insert(pos) {
                    if let Some(x) = do_calc(map, path, pos, dir, count) {
                        min = min.min(x + map[row][col]);
                    }
                }
            }
            // up
            if row > 0 {
                let row = row - 1;
                let dir = Dir::Up;
                let pos = Pos::new(row, col);
                let mut path = path.clone();
                if path.insert(pos) {
                    if let Some(x) = do_calc(map, path, pos, dir, count) {
                        min = min.min(x + map[row][col]);
                    }
                }
            }
            // down
            if row < heigth - 1 {
                let row = row + 1;
                let dir = Dir::Down;
                let pos = Pos::new(row, col);
                let mut path = path.clone();
                if path.insert(pos) {
                    if let Some(x) = do_calc(map, path, pos, dir, count) {
                        min = min.min(x + map[row][col]);
                    }
                }
            }
        },
    }

    if min == usize::MAX {None} else {Some(min)}
}

fn solve(input: &str) -> usize {
    let map = input.trim()
        .split("\n")
        .map(|line| line.chars().map(|c|c as usize - '0' as usize).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    calc(&map)
}

#[test]
fn test() {
    let input = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
    ".trim();

    println!("{}", input);

    let min = solve(input);
    println!("min: {}", min);
}
