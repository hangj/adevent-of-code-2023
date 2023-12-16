use std::collections::HashSet;


enum Tile {
    /// `.`
    Empty,
    /// `/` or `\`
    Mirror,
    /// `|` or `-`
    Splitter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    pub row: usize,
    pub col: usize,
    pub direction: Direction,
}

impl Beam {
    pub fn new(row: usize, col: usize, direction: Direction) -> Self {
        Self { row, col, direction }
    }

    pub fn heading_next(&self, contraption: &Contraption) -> Vec<Self> {
        let height = contraption.tiles.len();
        let width = contraption.tiles[0].len();

        let (row, col, dir) = (self.row, self.col, self.direction);
        let mut ret = Vec::with_capacity(2);

        let tile = contraption.tiles[row][col];
        match tile {
            '.' => {
                match dir {
                    Direction::Up => {
                        if row > 0 {
                            ret.push(Self::new(row-1, col, Direction::Up));
                        }
                    },
                    Direction::Down => {
                        if row < height - 1 {
                            ret.push(Self::new(row+1, col, Direction::Down));
                        }
                    },
                    Direction::Left => {
                        if col > 0 {
                            ret.push(Self::new(row, col-1, Direction::Left));
                        }
                    },
                    Direction::Right => {
                        if col < width - 1 {
                            ret.push(Self::new(row, col+1, Direction::Right));
                        }
                    },
                }
            },
            '|' => {
                if dir != Direction::Down && row > 0 {
                    ret.push(Self::new(row-1, col, Direction::Up));
                }
                if dir != Direction::Up && row < height - 1 {
                    ret.push(Self::new(row+1, col, Direction::Down));
                }
            },
            '-' => {
                if dir != Direction::Left && col < width - 1{
                    ret.push(Self::new(row, col+1, Direction::Right));
                }
                if dir != Direction::Right && col > 0  {
                    ret.push(Self::new(row, col-1, Direction::Left));
                }
            },
            '/' => {
                match dir {
                    Direction::Up => if col < width - 1 {
                        ret.push(Self::new(row, col+1, Direction::Right));
                    },
                    Direction::Down => if col > 0 {
                        ret.push(Self::new(row, col-1, Direction::Left));
                    },
                    Direction::Left => if row < height - 1 {
                        ret.push(Self::new(row+1, col, Direction::Down));
                    },
                    Direction::Right => if row > 0 {
                        ret.push(Self::new(row-1, col, Direction::Up));
                    },
                }
            },
            '\\' => {
                match dir {
                    Direction::Up => if col > 0 {
                        ret.push(Self::new(row, col-1, Direction::Left));
                    },
                    Direction::Down => if col < width - 1 {
                        ret.push(Self::new(row, col+1, Direction::Right));
                    },
                    Direction::Left => if row > 0 {
                        ret.push(Self::new(row-1, col, Direction::Up));
                    },
                    Direction::Right => if row < height - 1 {
                        ret.push(Self::new(row+1, col, Direction::Down));
                    },
                }
            },
            _ => unreachable!(),
        }
        ret
    }
}

struct Contraption {
    tiles: Vec<Vec<char>>,
}

impl Contraption {
    pub fn new(tiles: Vec<Vec<char>>) -> Self {
        Self { tiles }
    }
}

fn calc_energized_tiles(beam: Beam, contraption: &Contraption) -> usize {
    let mut hash = HashSet::new();
    hash.insert(beam);

    let mut v = beam.heading_next(&contraption);

    loop {
        if v.is_empty() { break; }

        let mut new_beams = Vec::new();
        for beam in v {
            if hash.get(&beam).is_none() {
                hash.insert(beam);
                new_beams.extend(beam.heading_next(&contraption).into_iter());
            }
        }
        v = new_beams;
    }

    let mut energized_nodes = HashSet::new();

    hash.iter()
        .map(|beam| {
            (beam.row, beam.col)
        })
        .for_each(|n| {energized_nodes.insert(n);});

    energized_nodes.len()
}

#[test]
fn test() {
    let input = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
".trim();

    let input = include_str!("input");
    println!("{}", input);

    let tiles = input.split("\n")
        .map(|line| {
            line.chars().collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let contraption = Contraption::new(tiles);

    let beam = Beam::new(0, 0, Direction::Right);
    let count = calc_energized_tiles(beam, &contraption);
    println!("energized nodes: {}", count);

    // Part Two
    let mut max = 0;
    let height = contraption.tiles.len();
    let width = contraption.tiles[0].len();

    for row in 0..height {
        let beam = Beam::new(row, 0, Direction::Right);
        let count = calc_energized_tiles(beam, &contraption);
        max = max.max(count);
        let beam = Beam::new(row, width-1, Direction::Left);
        let count = calc_energized_tiles(beam, &contraption);
        max = max.max(count);
    }
    for col in 0..width {
        let beam = Beam::new(0, col, Direction::Down);
        let count = calc_energized_tiles(beam, &contraption);
        max = max.max(count);
        let beam = Beam::new(height-1, col, Direction::Up);
        let count = calc_energized_tiles(beam, &contraption);
        max = max.max(count);
    }

    println!("max: {:?}", max);
}

