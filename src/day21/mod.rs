use std::collections::HashSet;




#[test]
fn test() {
    let input = {"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
    ".trim() 
    };

    // let input = include_str!("input");

    let map = input.split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let S = map.iter().enumerate()
        .find_map(|(row, v)| {
            if let Some(col) = v.iter().enumerate().find_map(|(col, ch)| if *ch=='S' {Some(col)}else{None}) {
                Some((row, col))
            } else {
                None
            }
        })
        .unwrap();
    println!("S: {:?}", S);

    let mut plots = HashSet::new();
    plots.insert(S);

    for _ in 0..64 {
        let mut new_plots = HashSet::new();
        for (row, col) in plots.iter() {
            let (row, col) = (*row, *col);
            // up
            if row > 0 {
                let row = row - 1;
                if map[row][col] != '#' {
                    new_plots.insert((row, col));
                }
            }
            // down
            if row < map.len() - 1 {
                let row = row + 1;
                if map[row][col] != '#' {
                    new_plots.insert((row, col));
                }
            }
            // left
            if col > 0 {
                let col = col - 1;
                if map[row][col] != '#' {
                    new_plots.insert((row, col));
                }
            }
            // right
            if col < map[0].len() - 1 {
                let col = col + 1;
                if map[row][col] != '#' {
                    new_plots.insert((row, col));
                }
            }
        }

        plots = new_plots;
    }

    println!("{}", plots.len());


    // Part Two
    {
        let len = map.len() as i32;
        let calc = |x: i32| {
            if x >= 0 {
                x % len
            } else {
                len - 1 - (-x - 1) % len
            }
        };

        let mut plots = HashSet::with_capacity(7000000);
        plots.insert((S.0 as i32, S.1 as i32));
        let mut new_plots = HashSet::with_capacity(700000);

        // 26501365 = 202300 * 131 + 65
        for i in 0..5+11+11+11+11+11 {
            println!("i: {}", i);
            new_plots.clear();
            for (row, col) in plots.iter() {
                let (row, col) = (*row, *col);
                // up
                {
                    let r = calc(row - 1);
                    let c = calc(col);

                    if map[r as usize][c as usize] != '#' {
                        new_plots.insert((row-1, col));
                    }
                }
                // down
                {
                    let r = calc(row + 1);
                    let c = calc(col);
                    if map[r as usize][c as usize] != '#' {
                        new_plots.insert((row+1, col));
                    }
                }
                // left
                {
                    let r = calc(row);
                    let c = calc(col - 1);
                    if map[r as usize][c as usize] != '#' {
                        new_plots.insert((row, col-1));
                    }
                }
                // right
                {
                    let r = calc(row);
                    let c = calc(col + 1);
                    if map[r as usize][c as usize] != '#' {
                        new_plots.insert((row, col+1));
                    }
                }
            }

            std::mem::swap(&mut plots, &mut new_plots);
        }

        println!("{}", plots.len());
    }
}
