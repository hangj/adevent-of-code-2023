use std::{collections::HashSet, hint};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    pub row: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    pub fn distance(&self, other: &Pos, empty_rows: &HashSet<usize>, empty_cols: &HashSet<usize>) -> usize {
        // each empty row should be replaced with 1000000 empty rows, and each empty column should be replaced with 1000000 empty columns.
        let factor = 1000000;

        let low = self.row.min(other.row);
        let high = self.row.max(other.row);
        let mut dist_horizontal = 0;

        for row in low..high {
            dist_horizontal += if empty_rows.get(&row).is_some() {
                factor
            } else {
                1
            }
        }

        let low = self.col.min(other.col);
        let high = self.col.max(other.col);
        let mut dist_vertical = 0;

        for col in low..high {
            dist_vertical += if empty_cols.get(&col).is_some() {
                factor
            } else {
                1
            }
        }

        dist_horizontal + dist_vertical
    }
}


#[test]
fn test() {
    let input = 
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    let input = include_str!("input");

    let mut map = Vec::new();
    let mut empty_rows = HashSet::new();
    let mut empty_cols = HashSet::new();

    input.split("\n")
        .enumerate()
        .for_each(|(r, line)| {
            let row = line.chars().collect::<Vec<_>>();
            if line.chars().all(|c|c!='#') {
                empty_rows.insert(r);
            }
            map.push(row);
        });
    
    for col in (0..map[0].len()).rev() {
        let mut is_empty_column = true;
        for row in 0..map.len() {
            if map[row][col] == '#' {
                is_empty_column = false;
                break;
            }
        }
        if is_empty_column {
            empty_cols.insert(col);
        }
    }

    let mut hash = HashSet::<Pos>::new();
    let mut sum = 0;

    map.iter()
        .enumerate()
        .for_each(|(row, line)| {
            line.iter()
                .enumerate()
                .for_each(|(col, c)| {
                    if *c != '#' { return; }

                    let pos = Pos::new(row, col);
                    sum += hash.iter().map(|p|p.distance(&pos, &empty_rows, &empty_cols)).sum::<usize>();

                    hash.insert(pos);
                });
        });

    println!("sum: {:?}", sum);
}