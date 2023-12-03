use std::{ops::{Range, Index}, collections::{HashSet, HashMap}};


#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Num {
    pub row: usize,
    pub range: Range<usize>,
}

impl Num {
    pub fn new(row: usize, range: Range<usize>) -> Self {
        Self { row, range }
    }
}

fn find_start_end(line: &str, idx: usize) -> Range<usize> {
    let mut forward = idx;
    let mut backward = idx;
    for (i, c) in line[..=idx].chars().rev().enumerate() {
        if !c.is_ascii_digit() {
            break;
        }
        backward = idx - i;
    }

    for (i, c) in line[idx..].chars().enumerate() {
        if !c.is_ascii_digit() {
            break;
        }
        forward = idx + 1 + i;
    }

    backward..forward
}


/// 找到 arr[row][col] 上下左右的 number
fn find_num(arr: &Vec<&str>, row: usize, col: usize) -> HashSet<Num> {
    let mut all = HashSet::new();

    // -------------- Up ---------------
    if row > 0 {
        let line = arr[ row-1 ];

        let rng = find_start_end(line, col);
        if !rng.is_empty() {
            all.insert(Num::new(row-1, rng));
        }

        if col > 0 {
            let rng = find_start_end(line, col-1);
            if !rng.is_empty() {
                all.insert(Num::new(row-1, rng));
            }
        }

        if col < line.len() - 1 {
            let rng = find_start_end(line, col+1);
            if !rng.is_empty() {
                all.insert(Num::new(row-1, rng));
            }
        }
    }

    // -------------- Down ---------------
    if row < arr.len() - 1 {
        let line = arr[ row+1 ];

        let rng = find_start_end(line, col);
        if !rng.is_empty() {
            all.insert(Num::new(row+1, rng));
        }

        if col > 0 {
            let rng = find_start_end(line, col-1);
            if !rng.is_empty() {
                all.insert(Num::new(row+1, rng));
            }
        }

        if col < line.len() - 1 {
            let rng = find_start_end(line, col+1);
            if !rng.is_empty() {
                all.insert(Num::new(row+1, rng));
            }
        }
    }

    // -------------- Left ---------------
    let line = arr[row];
    if col > 0 {
        let rng = find_start_end(line, col - 1);
        if !rng.is_empty() {
            all.insert(Num::new(row, rng));
        }
    }

    // -------------- Right ---------------
    if col < line.len() - 1 {
        let rng = find_start_end(line, col + 1);
        if !rng.is_empty() {
            all.insert(Num::new(row, rng));
        }
    }

    all
}

#[test]
fn solve() {
    let input = include_str!("input");
    let arr = input.split("\n")
        .collect::<Vec<_>>();

    let mut all_rngs = HashSet::new();

    for (row, line) in arr.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() { continue; }
            if c == '.' { continue; }

            let rngs = find_num(&arr, row, col);
            for rng in rngs {
                all_rngs.insert(rng);
            }
        }
    }

    let mut sum = 0;
    for rng in all_rngs {
        let line = arr[rng.row];
        let d: u32 = line.index(rng.range).parse().unwrap();
        sum += d;
    }

    println!("sum: {}", sum);


    // ------- Part Two -----------
    let mut sum = 0;

    for (row, line) in arr.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '*' { continue; }

            let rngs = find_num(&arr, row, col);
            if rngs.len() != 2 { continue; }

            let mut it = rngs.iter();
            let gear_first = it.next().unwrap();
            let gear_second = it.next().unwrap();

            let first: u32 = arr[gear_first.row].index(gear_first.range.clone()).parse().unwrap();
            let second: u32 = arr[gear_second.row].index(gear_second.range.clone()).parse().unwrap();
            sum += first * second;
        }
    }

    println!("sum: {}", sum);

}

#[test]
fn test_find_start_end() {
    let s = "hello1234df";
    let rng = find_start_end(s, 9);
    println!("rng: {:?}", rng);
    println!("{}", s.index(rng));
}

#[test]
fn test_find_num() {
    let vec = vec!["hello1h234world", "hello1o234world", "hello1234world"];
    let ret = find_num(&vec, 1, 6);
    println!("ret: {:#?}", ret);
}