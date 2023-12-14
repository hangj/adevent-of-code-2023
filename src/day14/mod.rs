use std::{cmp::Ordering, collections::HashMap};


fn tilt_north(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut v = rotate(v);

    for v in v.iter_mut() {
        sort(v, |a, b| b.cmp(a));
    }

    rotate(v)
}

fn tilt_west(mut v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for v in v.iter_mut() {
        sort(v, |a, b| b.cmp(a));
    }
    v
}

fn tilt_south(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut v = rotate(v);

    for v in v.iter_mut() {
        sort(v, |a, b| a.cmp(b));
    }

    rotate(v)
}

fn tilt_east(mut v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for v in v.iter_mut() {
        sort(v, |a, b| a.cmp(b));
    }
    v
}

fn rotate<T>(vec2d: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut ret = Vec::with_capacity(vec2d[0].len());

    for v in vec2d.into_iter() {
        for (i, v) in v.into_iter().enumerate() {
            if i >= ret.len() {
                ret.push(Vec::new());
            }
            ret[i].push(v);
        }
    }
    ret
}

///  # 35
///  . 46
///  O 79
fn sort<F>(v: &mut [char], compare: F)
where
    F: Fn(&char, &char) -> Ordering,
{
    let mut start = 0;

    for i in 0..v.len() {
        if v[i] == '#' {
            if start < i {
                v[start..i].sort_by(&compare);
            }
            start = i+1;
        }
    }

    if start < v.len() {
        v[start..].sort_by(compare);
    }
}


#[test]
fn test() {
    let input = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
    .trim();

    let input = include_str!("input");

    let v = input.split("\n")
        .map(|line| line.chars()
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let origin = v.clone();

    let v = tilt_north(v);

    let mut load = v.len();
    let mut sum = 0;

    for v in v.iter() {
        // println!("{:?}", v);
        sum += v.iter().filter(|c|**c=='O').count() * load;
        load -= 1;
    }

    println!("sum: {:?}", sum);

    // Part Two
    let mut v = origin;
    let cycles = 1000000000;

    let mut states = Vec::new();
    let mut hash = HashMap::new();

    let mut idx_state = None;
    let mut idx_i = 0;

    for i in 0..cycles {
        v = tilt_north(v);
        v = tilt_west(v);
        v = tilt_south(v);
        v = tilt_east(v);

        if let Some(j) = hash.get(&v) {
            idx_state = Some(*j);
            idx_i = i;
            break;
        }
        states.push(v.clone());
        hash.insert(v.clone(), states.len()-1);
    }

    if let Some(idx) = idx_state {
        let len = states.len() - idx;
        let diff = cycles - idx_i - 1;

        v = states[idx + diff % len].clone();
    }


    let mut load = v.len();
    let mut sum = 0;

    for v in v.iter() {
        println!("{:?}", v);
        sum += v.iter().filter(|c|**c=='O').count() * load;
        load -= 1;
    }

    println!("sum: {:?}", sum);


}