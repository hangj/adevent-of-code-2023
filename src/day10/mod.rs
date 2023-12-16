use std::{collections::{HashMap, HashSet}, sync::{Arc, Weak}};

#[derive(Debug)]
pub struct Node {
    pub pipe: Option<Pipe>,
    pub is_border: bool,
    pub outside_dir: Option<Direction>,
}

impl Node {
    // 外侧是否在内角的方向
    fn is_outside_at_inner_angle(&self) -> Result<bool, ()> {
        let pipe = self.pipe.unwrap();
        if pipe == Pipe::Vertical || pipe == Pipe::Horizontal {
            return Err(());
        }
        let dir = self.outside_dir.unwrap();

        match pipe {
            Pipe::Start | Pipe::Vertical | Pipe::Horizontal => return Err(()),
            Pipe::L => Ok(dir == Direction::Up || dir == Direction::Right),
            Pipe::J => Ok(dir == Direction::Up || dir == Direction::Left),
            Pipe::P7 => Ok(dir == Direction::Down || dir == Direction::Left),
            Pipe::F => Ok(dir == Direction::Down || dir == Direction::Right),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    /// 'S'
    Start,
    /// '|'
    Vertical,
    /// '-'
    Horizontal,
    /// 'L'
    L,
    /// 'J'
    J,
    /// '7'
    P7,
    /// 'F'
    F,
}

impl Pipe {
    pub fn hash_up(&self) -> bool {
        match *self {
            Self::Vertical | Self::L | Self::J => true,
            _ => false,
        }
    }
    pub fn hash_down(&self) -> bool {
        match *self {
            Self::Vertical | Self::F | Self::P7 => true,
            _ => false,
        }
    }
    pub fn hash_left(&self) -> bool {
        match *self {
            Self::Horizontal | Self::P7 | Self::J => true,
            _ => false,
        }
    }
    pub fn hash_right(&self) -> bool {
        match *self {
            Self::Horizontal | Self::L | Self::F => true,
            _ => false,
        }
    }
    pub fn get_random_direction(&self) -> Direction {
        match self {
            Pipe::Vertical => Direction::Up,
            Pipe::Horizontal => Direction::Left,
            Pipe::L => Direction::Up,
            Pipe::J => Direction::Up,
            Pipe::P7 => Direction::Left,
            Pipe::F => Direction::Down,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::L),
            'J' => Ok(Self::J),
            'F' => Ok(Self::F),
            '7' => Ok(Self::P7),
            _ => Err(()),
        }
    }
}

pub struct Maze {
    pub maze: Vec<Vec<Node>>,
    pub start_row: usize,
    pub start_col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Idx {
    pub row: usize,
    pub col: usize,
    /// 上一个 node 在当前 node 的哪一边
    pub from_dir: Option<Direction>,
}

impl Idx {
    pub fn new(row: usize, col: usize, from_dir: Option<Direction>) -> Self {
        Self { row, col, from_dir }
    }
}

impl Maze {
    pub fn new(maze: Vec<Vec<Node>>, start_row: usize, start_col: usize) -> Self {
        let mut this = Self { maze, start_row, start_col };
        this.find_start();
        for (node, _dir) in this.iter_mut() {
            node.is_border = true;
        }
        this
    }

    pub fn is_border(&self, row: usize, col: usize) -> bool {
        self.maze[row][col].is_border
    }

    pub fn iter(&self) -> Iter<'_> {
        let (row, col) = (self.start_row, self.start_col);
        Iter { maze: self, idx: Idx::new(row, col, None) }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        let (row, col) = (self.start_row, self.start_col);
        IterMut::new(self, Idx::new(row, col, None))
    }
    pub fn new_iter_mut(&mut self, row: usize, col: usize) -> IterMut<'_> {
        IterMut::new(self, Idx::new(row, col, None))
    }

    fn find_start(&mut self) -> Vec<Idx> {
        let maze = &mut self.maze;
        let mut ret = Vec::new();

        let row = self.start_row;
        let col = self.start_col;

        let up = if row > 0 { maze[row-1][col].pipe } else { None };
        let down = if row < maze.len()-1 { maze[row+1][col].pipe } else { None };
        let left = if col > 0 { maze[row][col-1].pipe } else { None };
        let right = if col < maze[0].len()-1 { maze[row][col+1].pipe } else { None };

        let node = &mut maze[row][col];

        let mut hash_up = false;
        let mut hash_down = false;
        let mut hash_left = false;
        let mut hash_right = false;

        if up.is_some() && up.unwrap().hash_down() {
            ret.push(Idx::new(row-1, col, Some(Direction::Down)));
            hash_up = true;
        }
        if down.is_some() && down.unwrap().hash_up() {
            ret.push(Idx::new(row+1, col, Some(Direction::Up)));
            hash_down = true;
        }
        if left.is_some() && left.unwrap().hash_right() {
            ret.push(Idx::new(row, col-1, Some(Direction::Right)));
            hash_left = true;
        }
        if right.is_some() && right.unwrap().hash_left() {
            ret.push(Idx::new(row, col+1, Some(Direction::Left)));
            hash_right = true;
        }

        if hash_left && hash_right {
            node.pipe = Some(Pipe::Horizontal);
        }
        if hash_up && hash_down {
            node.pipe = Some(Pipe::Vertical);
        }
        if hash_up && hash_left {
            node.pipe = Some(Pipe::J);
        }
        if hash_down && hash_right {
            node.pipe = Some(Pipe::F);
        }
        if hash_up && hash_right {
            node.pipe = Some(Pipe::L);
        }
        if hash_down && hash_left {
            node.pipe = Some(Pipe::P7);
        }

        ret
    }

    fn find_path(&self, idx: Idx) -> Option<Idx> {
        let row = idx.row;
        let col = idx.col;
        // 前一个 node 在这个 node 的哪一边
        let from_dir = idx.from_dir;

        let maze = &self.maze;

        let up = if row > 0 { maze[row-1][col].pipe } else { None };
        let down = if row < maze.len()-1 { maze[row+1][col].pipe } else { None };
        let left = if col > 0 { maze[row][col-1].pipe } else { None };
        let right = if col < maze[row].len()-1 { maze[row][col+1].pipe } else { None };
        let pipe = maze[row][col].pipe.unwrap();

        match pipe {
            // '-'
            Pipe::Horizontal => {
                if from_dir.unwrap() == Direction::Right && left.unwrap().hash_right() {
                    return Some(Idx::new(row, col-1, Some(Direction::Right)));
                }
                else if from_dir.unwrap() == Direction::Left && right.unwrap().hash_left() {
                    return Some(Idx::new(row, col+1, Some(Direction::Left)));
                }
                None
            },
            // '|'
            Pipe::Vertical => {
                if from_dir.unwrap() == Direction::Down && up.unwrap().hash_down() {
                    return Some(Idx::new(row-1, col, Some(Direction::Down)));
                }
                if from_dir.unwrap() == Direction::Up && down.unwrap().hash_up() {
                    return Some(Idx::new(row+1, col, Some(Direction::Up)));
                }
                None
            },
            Pipe::J => {
                if from_dir.unwrap() == Direction::Left && up.unwrap().hash_down() {
                    return Some(Idx::new(row-1, col, Some(Direction::Down)));
                }
                if from_dir.unwrap() == Direction::Up && left.unwrap().hash_right() {
                    return Some(Idx::new(row, col-1, Some(Direction::Right)));
                }
                None
            },
            Pipe::F => {
                if from_dir.unwrap() == Direction::Right && down.unwrap().hash_up() {
                    return Some(Idx::new(row+1, col, Some(Direction::Up)));
                }
                if from_dir.unwrap() == Direction::Down && right.unwrap().hash_left() {
                    return Some(Idx::new(row, col+1, Some(Direction::Left)));
                }
                None
            },
            Pipe::L => {
                if from_dir.unwrap() == Direction::Right && up.unwrap().hash_down() {
                    return Some(Idx::new(row-1, col, Some(Direction::Down)));
                }
                if from_dir.unwrap() == Direction::Up && right.unwrap().hash_left() {
                    return Some(Idx::new(row, col+1, Some(Direction::Left)));
                }
                None
            },
            Pipe::P7 => {
                if from_dir.unwrap() == Direction::Left && down.unwrap().hash_up() {
                    return Some(Idx::new(row+1, col, Some(Direction::Up)));
                }
                if from_dir.unwrap() == Direction::Down && left.unwrap().hash_right() {
                    return Some(Idx::new(row, col-1, Some(Direction::Right)));
                }
                None
            },
            _ => None,
        }
    }

    fn update_outside_direction(&mut self, row: usize, col: usize, direction: Direction) {
        self.maze[row][col].outside_dir = Some(direction);

        let mut it = self.new_iter_mut(row, col);
        let (mut pre_node, _) = it.next().unwrap();
        for (cur_node, idx) in it {
            let dir = idx.from_dir.unwrap();
            let prev_pipe = pre_node.pipe.unwrap();
            let cur_pipe = cur_node.pipe.unwrap();
            let pre_outside_dir = pre_node.outside_dir.unwrap();

            if prev_pipe == Pipe::Vertical || prev_pipe == Pipe::Horizontal {
                cur_node.outside_dir = Some(pre_outside_dir);
            } else {
                // 内角是否为外侧
                let inner_side = pre_node.is_outside_at_inner_angle().unwrap();
                match prev_pipe {
                    Pipe::L => {
                        // x
                        // L
                        if dir == Direction::Down {
                            cur_node.outside_dir = Some(
                                if inner_side {
                                    Direction::Right
                                } else {
                                    Direction::Left
                                });
                        } else {
                            // L x
                            cur_node.outside_dir = Some(
                                if inner_side {
                                    Direction::Up
                                } else {
                                    Direction::Down
                                });
                        }
                    },
                    Pipe::J => {
                        // x
                        // J
                        if dir == Direction::Down {
                            cur_node.outside_dir = Some(
                                if inner_side {
                                    Direction::Left
                                } else {
                                    Direction::Right
                                });
                        } else {
                            // x J
                            cur_node.outside_dir = Some(
                                if inner_side {
                                    Direction::Up
                                } else {
                                    Direction::Down
                                });
                        }
                    },
                    Pipe::P7 => {
                        // 7
                        // x
                        if dir == Direction::Up {
                            cur_node.outside_dir = Some(
                                if inner_side {
                                    Direction::Left
                                } else {
                                    Direction::Right
                                });
                        } else {
                            // x 7
                            cur_node.outside_dir = Some(
                                if inner_side {
                                    Direction::Down
                                } else {
                                    Direction::Up
                                });
                        }
                    },
                    Pipe::F => {
                        // F
                        // x
                        if dir == Direction::Up {
                            cur_node.outside_dir = Some(
                                if inner_side {
                                    Direction::Right
                                } else {
                                    Direction::Left
                                });
                        } else {
                            // F x
                            cur_node.outside_dir = Some(
                                if inner_side {
                                    Direction::Down
                                } else {
                                    Direction::Up
                                });
                        }
                    },
                    _ => unreachable!(),
                }
            }
            pre_node = cur_node;
        }
    }

    fn calc_inner_count(&self) -> usize {
        let mut seed_nodes = HashSet::new();
        let height = self.maze.len();
        let width = self.maze[0].len();

        for (node, idx) in self.iter() {
            let pipe = node.pipe.unwrap();
            if let Ok(is_inner) = node.is_outside_at_inner_angle() {
                match pipe {
                    Pipe::L => if is_inner {
                            // left
                            if idx.col > 0 {
                                seed_nodes.insert((idx.row, idx.col - 1));
                            }
                            // down
                            if idx.row < height - 1 {
                                seed_nodes.insert((idx.row + 1, idx.col));
                            }
                        } else {
                            // right
                            if idx.col < width - 1 {
                                seed_nodes.insert((idx.row, idx.col + 1));
                            }
                            // up
                            if idx.row > 0 {
                                seed_nodes.insert((idx.row - 1, idx.col));
                            }
                        },
                    Pipe::J => if is_inner {
                            // right
                            if idx.col < width - 1 {
                                seed_nodes.insert((idx.row, idx.col + 1));
                            }
                            // down
                            if idx.row < height - 1 {
                                seed_nodes.insert((idx.row + 1, idx.col));
                            }
                        } else {
                            // left
                            if idx.col > 0 {
                                seed_nodes.insert((idx.row, idx.col - 1));
                            }
                            // up
                            if idx.row > 0 {
                                seed_nodes.insert((idx.row - 1, idx.col));
                            }
                        },
                    Pipe::P7 => if is_inner {
                            // right
                            if idx.col < width - 1 {
                                seed_nodes.insert((idx.row, idx.col + 1));
                            }
                            // up
                            if idx.row > 0 {
                                seed_nodes.insert((idx.row - 1, idx.col));
                            }
                        } else {
                            // left
                            if idx.col > 0 {
                                seed_nodes.insert((idx.row, idx.col - 1));
                            }
                            // down
                            if idx.row < height - 1 {
                                seed_nodes.insert((idx.row + 1, idx.col));
                            }
                        },
                    Pipe::F => if is_inner {
                            // left
                            if idx.col > 0 {
                                seed_nodes.insert((idx.row, idx.col - 1));
                            }
                            // up
                            if idx.row > 0 {
                                seed_nodes.insert((idx.row - 1, idx.col));
                            }
                        } else {
                            // right
                            if idx.col < width - 1 {
                                seed_nodes.insert((idx.row, idx.col + 1));
                            }
                            // down
                            if idx.row < height - 1 {
                                seed_nodes.insert((idx.row + 1, idx.col));
                            }
                        },
                    _ => unreachable!(),
                }
            } else {
                let dir = node.outside_dir.unwrap();
                let (row, col) = match dir {
                    Direction::Up => {
                        if idx.row >= height - 1 { continue; }
                        (idx.row + 1, idx.col)
                    },
                    Direction::Down => {
                        if idx.row <= 0 { continue; }
                        (idx.row - 1, idx.col)
                    },
                    Direction::Left => {
                        if idx.col >= width - 1 { continue; }
                        (idx.row, idx.col + 1)
                    },
                    Direction::Right => {
                        if idx.col <= 0 { continue; }
                        (idx.row, idx.col - 1)
                    },
                };
                seed_nodes.insert((row, col));
            };
        }

        seed_nodes = seed_nodes.into_iter()
            .filter(|(row, col)| !self.is_border(*row, *col))
            .collect();

        self.traverse_inner(&mut seed_nodes);
        seed_nodes.len()
    }

    fn traverse_inner(&self, inner_nodes: &mut HashSet<(usize, usize)>) {
        let height = self.maze.len();
        let width = self.maze[0].len();
        let mut new_found_nodes = inner_nodes.clone();

        loop {
            if new_found_nodes.is_empty() {
                break;
            }
            let mut nodes = HashSet::new();

            for (row, col) in new_found_nodes.into_iter() {
                // up
                if row > 0 {
                    let row = row-1;
                    let col = col;
                    if !self.is_border(row, col) && inner_nodes.get(&(row, col)).is_none() {
                        inner_nodes.insert((row, col));
                        nodes.insert((row, col));
                    }
                }
                // down
                if row < height - 1 {
                    let row = row+1;
                    let col = col;
                    if !self.is_border(row, col) && inner_nodes.get(&(row, col)).is_none() {
                        inner_nodes.insert((row, col));
                        nodes.insert((row, col));
                    }
                }
                // left
                if col > 0 {
                    let row = row;
                    let col = col - 1;
                    if !self.is_border(row, col) && inner_nodes.get(&(row, col)).is_none() {
                        inner_nodes.insert((row, col));
                        nodes.insert((row, col));
                    }
                }
                // right
                if col < width - 1 {
                    let row = row;
                    let col = col + 1;
                    if !self.is_border(row, col) && inner_nodes.get(&(row, col)).is_none() {
                        inner_nodes.insert((row, col));
                        nodes.insert((row, col));
                    }
                }
            }
            new_found_nodes = nodes;
        }
    }
}

pub struct Iter<'a> {
    pub maze: &'a Maze,
    pub idx: Idx,

    // start_idx: Idx,
    // started: bool,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a Node, Idx);

    fn next(&mut self) -> Option<Self::Item> {
        let mut idx = self.idx;
        let (row, col) = (idx.row, idx.col);
        if idx.from_dir.is_some() && row == self.maze.start_row && col == self.maze.start_col {
            return None;
        }

        let node = &self.maze.maze[row][col];

        if idx.from_dir.is_none() {
           idx.from_dir = Some(node.pipe.unwrap().get_random_direction());
        }

        self.idx = self.maze.find_path(idx).unwrap();

        Some((node, idx))
    }
}

pub struct IterMut<'a> {
    pub maze: &'a mut Maze,
    pub idx: Idx,

    start_idx: Idx,
    started: bool,
}

impl<'a> IterMut<'a> {
    pub fn new(maze: &'a mut Maze, idx: Idx) -> Self {
        Self { maze, idx, start_idx: idx, started: false }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (&'a mut Node, Idx);

    fn next(&mut self) -> Option<Self::Item> {
        let mut idx = self.idx;
        let (row, col) = (idx.row, idx.col);

        if row == self.start_idx.row && col == self.start_idx.col && self.started {
            return None;
        }
        self.started = true;

        let node = &self.maze.maze[row][col];

        if idx.from_dir.is_none() {
           idx.from_dir = Some(node.pipe.unwrap().get_random_direction());
        }
        self.idx = self.maze.find_path(idx).unwrap();

        let node = unsafe {
            &mut *{ &mut self.maze.maze[row][col] as *mut _ }
        };

        Some((node, idx))
    }
}



#[test]
fn solve() {
    let input = "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    let input = include_str!("input");

    let mut start_row = 0;
    let mut start_col = 0;

    let maze = input.split("\n")
        .filter(|s|!s.is_empty())
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    let node = Node {
                        pipe: Pipe::try_from(c).ok(),
                        is_border: false,
                        outside_dir: None,
                    };
                    if let Some(Pipe::Start) = node.pipe {
                        start_row = row;
                        start_col = col;
                    }
                    node
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut maze = Maze::new(maze, start_row, start_col);

    let node_count = maze.iter().count();
    println!("node_count: {}", node_count);

    println!("steps: {}", (node_count+1)/2);


    // Part Two
    // 先找到一个一定处于外面的点
    let len = maze.maze.len();

    let mut outer_idx = None;
    for i in 0..len {
        if !maze.is_border(i, 0) {
            outer_idx = Some((i, 0));
            break;
        }
        if !maze.is_border(i, len-1) {
            outer_idx = Some((i, len-1));
            break;
        }
        if !maze.is_border(0, i) {
            outer_idx = Some((0, i));
            break;
        }
        if !maze.is_border(len-1, i) {
            outer_idx = Some((len-1, i));
            break;
        }
    }

    let idx = outer_idx.unwrap();
    let vec = &mut maze.maze;
    let idx = find_maze_border(vec, idx);
    let (row, col) = (idx.row, idx.col);

    maze.update_outside_direction(row, col, idx.from_dir.unwrap());
    let inner_count = maze.calc_inner_count();
    println!("inner_count: {}", inner_count);
}

/// 由外向内, 找到第一个 maze 的边界, 并以此确定那边为外侧
fn find_maze_border(maze: &mut Vec<Vec<Node>>, idx: (usize, usize)) -> Idx {
    let (row, col) = idx;
    maze[row][col].pipe = None;
    let len = maze.len();

    let mut indices = HashSet::new();
    indices.insert((row, col));

    let mut start_row = 0;
    let mut start_col = 0;
    let mut from_dir = Direction::Down;

    'outer: loop {
        if indices.is_empty() {
            break;
        }
        let mut new_indices = HashSet::new();
        for (row, col) in indices.iter() {
            let (row, col) = (*row, *col);
            // up
            if row > 0 && maze[row-1][col].pipe.is_some() {
                let row = row - 1;
                if !maze[row][col].is_border {
                    maze[row][col].pipe = None;
                    new_indices.insert((row, col));
                } else {
                    start_row = row;
                    start_col = col;
                    from_dir = Direction::Down;
                    break 'outer;
                }
            }
            // down
            if row < len - 1 && maze[row+1][col].pipe.is_some() {
                let row = row + 1;
                if !maze[row][col].is_border {
                    maze[row][col].pipe = None;
                    new_indices.insert((row, col));
                } else {
                    start_row = row;
                    start_col = col;
                    from_dir = Direction::Up;
                    break 'outer;
                }
            }
            // left
            if col > 0 && maze[row][col-1].pipe.is_some() {
                let col = col - 1;
                if !maze[row][col].is_border {
                    maze[row][col].pipe = None;
                    new_indices.insert((row, col));
                } else {
                    start_row = row;
                    start_col = col;
                    from_dir = Direction::Right;
                    break 'outer;
                }
            }
            // right
            if col < len-1 && maze[row][col+1].pipe.is_some() {
                let col = col + 1;
                if !maze[row][col].is_border {
                    maze[row][col].pipe = None;
                    new_indices.insert((row, col));
                } else {
                    start_row = row;
                    start_col = col;
                    from_dir = Direction::Left;
                    break 'outer;
                }
            }
        }
        indices = new_indices;
    }

    Idx::new(start_row, start_col, Some(from_dir))
}

