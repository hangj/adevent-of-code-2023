#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MirrorType{
    LeftRight,
    UpDown,
}

// up or left count
fn fuck_1(group: &Vec<Vec<char>>) -> (MirrorType, usize) {
    let height = group.len();
    let width = group[0].len();

    for idx in 1..height {
        let mut up = idx - 1;
        let mut down = idx;

        let mut found = true;

        loop {
            if group[up] != group[down] {
                found = false;
                break;
            }

            if up == 0 || down == height-1 {break;}

            up -= 1;
            down += 1;
        }

        if found {
            return (MirrorType::UpDown, idx);
        }
    }

    for idx in 1..width {
        let mut left = idx - 1;
        let mut right = idx;
        let mut found = true;

        loop {
            if group.iter()
                .any(|vec|vec[left] != vec[right]) {
                    found = false;
                    break;
            }
            if left == 0 || right == width - 1 { break; }
            left -= 1;
            right += 1;
        }

        if found {
            return (MirrorType::LeftRight, idx);
        }
    }

    (MirrorType::LeftRight, 0)
}

fn fuck_2(group: &Vec<Vec<char>>) -> (MirrorType, usize) {
    let height = group.len();
    let width = group[0].len();

    for idx in 1..height {
        let mut up = idx - 1;
        let mut down = idx;

        let mut smudge_count = 0;

        loop {
            smudge_count += group[up].iter().zip(group[down].iter())
                .map(|(a, b)|if a!=b{1}else{0})
                .sum::<usize>();

            if smudge_count > 1 { break;}

            if up == 0 || down == height-1 {break;}

            up -= 1;
            down += 1;
        }

        if smudge_count == 1 {
            return (MirrorType::UpDown, idx);
        }
    }

    for idx in 1..width {
        let mut left = idx - 1;
        let mut right = idx;
        let mut smudge_count = 0;

        loop {
            smudge_count += group.iter()
                .map(|vec|if vec[left] != vec[right]{1}else{0})
                .sum::<usize>();

            if smudge_count > 1 { break;}

            if left == 0 || right == width - 1 { break; }
            left -= 1;
            right += 1;
        }

        if smudge_count == 1 {
            return (MirrorType::LeftRight, idx);
        }
    }

    (MirrorType::LeftRight, 0)
}

fn part_1(input: &str) -> usize {
    input.split("\n\n")
        .map(|group| {
            let vec = group.split("\n")
                .filter(|line|!line.is_empty())
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            // up * 100 + left
            let (t, n) = fuck_1(&vec);
            if n == 0 {
                println!("error: {}", group);
                panic!("error: {}", group);
            }

            if t == MirrorType::UpDown {
                n * 100
            } else {
                n
            }
        })
        .sum::<usize>()
}

fn part_2(input: &str) -> usize {
    input.split("\n\n")
        .map(|group| {
            let vec = group.split("\n")
                .filter(|line|!line.is_empty())
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let (t, n) = fuck_2(&vec);
            if n == 0 {
                println!("error: {}", group);
                panic!("error: {}", group);
            }

            if t == MirrorType::UpDown {
                n * 100
            } else {
                n
            }
        })
        .sum::<usize>()
}

#[test]
fn solve() {
    let input = include_str!("input");
    let sum = part_1(input);
    println!("sum: {:?}", sum);
    let sum = part_2(input);
    println!("sum: {:?}", sum);
}

#[test]
fn test() {
    let input = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    let sum = part_1(input);
    println!("sum: {:?}", sum);
    assert_eq!(sum, 405);

    let sum = part_2(input);
    println!("sum: {:?}", sum);
    assert_eq!(sum, 400);
}
