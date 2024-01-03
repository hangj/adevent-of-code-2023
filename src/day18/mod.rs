use std::{str::FromStr, collections::{HashSet, HashMap}};

mod part2;

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Dir::Up),
            "D" => Ok(Dir::Down),
            "L" => Ok(Dir::Left),
            "R" => Ok(Dir::Right),
            _ => Err(()),
        }
    }
}


struct DigCmd {
    dir: Dir,
    steps: i32,
}

impl DigCmd {
    fn new(dir: Dir, steps: i32) -> Self {
        Self { dir, steps }
    }
}

#[test]
fn test() {
    let input = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
    ".trim();

    // let input = include_str!("input");

    let cmds = input.split("\n")
        .map(|line| {
            let mut it = line.split(" ");
            let dir: Dir = it.next().unwrap().parse().unwrap();
            let steps: i32 = it.next().unwrap().parse().unwrap();
            let color = it.next().unwrap();
            let color = &color[2..8];

            let p2_steps = i32::from_str_radix(&color[..5], 16).unwrap();
            let p2_dir = match &color[5..] {
                "0" => Dir::Right,
                "1" => Dir::Down,
                "2" => Dir::Left,
                "3" => Dir::Up,
                _ => unreachable!(),
            };

            // DigCmd::new(dir, steps)
            DigCmd::new(p2_dir, p2_steps)
        })
        .collect::<Vec<_>>();

    let mut row = 0_i32;
    let mut col = 0_i32;
    let mut hash = HashSet::new();
    hash.insert((row, col));

    let mut left_most = 0;
    let mut right_most = 0;
    let mut up_most = 0;
    let mut down_most = 0;

    cmds.iter().for_each(|cmd| {
        let mut diff_row = 0;
        let mut diff_col = 0;
        match cmd.dir {
            Dir::Up => diff_row = -1,
            Dir::Down => diff_row = 1,
            Dir::Left => diff_col = -1,
            Dir::Right => diff_col = 1,
        }

        for _ in 0..cmd.steps {
            row += diff_row;
            col += diff_col;
            hash.insert((row, col));

            left_most = left_most.min(col);
            right_most = right_most.max(col);
            up_most = up_most.min(row);
            down_most = down_most.max(row);
        }
    });

    let mut borders = HashSet::new();

    {
        let left_most = left_most - 1;
        let right_most = right_most + 1;
        let up_most = up_most - 1;
        let down_most = down_most + 1;

        for row in up_most..=down_most {
            borders.insert((row, left_most));
            borders.insert((row, right_most));
        }

        for col in left_most..=right_most {
            borders.insert((up_most, col));
            borders.insert((down_most, col));
        }
    }

    {
        let mut outers = HashSet::new();

        for row in up_most..=down_most {
            if hash.get(&(row, left_most)).is_none() {
                borders.insert((row, left_most));
                outers.insert((row, left_most));
            }

            if hash.get(&(row, right_most)).is_none() {
                borders.insert((row, right_most));
                outers.insert((row, right_most));
            }
        }

        for col in left_most..=right_most {
            if hash.get(&(up_most, col)).is_none() {
                borders.insert((up_most, col));
                outers.insert((up_most, col));
            }
            if hash.get(&(down_most, col)).is_none() {
                borders.insert((down_most, col));
                outers.insert((down_most, col));
            }
        }

        loop {
            if outers.is_empty() { break; }

            let mut new_outers = HashSet::new();

            for (row, col) in outers.into_iter() {
                // up
                {
                    let row =row-1;
                    let pos = (row, col);
                    if hash.get(&pos).is_none() && borders.get(&pos).is_none() {
                        borders.insert(pos);
                        new_outers.insert(pos);
                    }
                }
                // down
                {
                    let row = row+1;
                    let pos = (row, col);
                    if hash.get(&pos).is_none() && borders.get(&pos).is_none() {
                        borders.insert(pos);
                        new_outers.insert(pos);
                    }
                }
                // left
                {
                    let col = col - 1;
                    let pos = (row, col);
                    if hash.get(&pos).is_none() && borders.get(&pos).is_none() {
                        borders.insert(pos);
                        new_outers.insert(pos);
                    }
                }
                // right
                {
                    let col = col + 1;
                    let pos = (row, col);
                    if hash.get(&pos).is_none() && borders.get(&pos).is_none() {
                        borders.insert(pos);
                        new_outers.insert(pos);
                    }
                }
            }

            outers = new_outers;
        }
    }

    let len = borders.len() as i32;
    let width = right_most + 1 - (left_most - 1) + 1;
    let height = down_most + 1 - (up_most - 1) + 1;

    println!("sum: {}", width * height - len);
}