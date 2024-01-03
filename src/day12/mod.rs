#![allow(unused_variables)]

use permutator::Combination;

fn calc(mut pre: String, record: &str, groups: &[usize]) -> usize {

    let mut new_record = None;

    for (i, c) in record.chars().enumerate() {
        if c == '?' {
            if i < record.len() - 1 {
                new_record = Some(&record[i+1..]);
            }
            break;
        }
        pre.push(c);
    }

    if new_record.is_none() {
        return if pre.chars().chain(record.chars())
            .collect::<String>()
            .split(".")
            .filter(|s|!s.is_empty())
            .map(|s|s.len())
            .zip(groups.iter())
            .all(|(a, b)| a == *b) {
                1
            } else {
                0
            };
    }

    let record = new_record.unwrap();

    let mut dot = pre.clone();
    dot.push('.');

    let mut pound = pre;
    pound.push('#');

    calc(dot, record, groups) + calc(pound, record, groups)
}

fn is_match(cond: &[char], records: &[usize]) -> bool {
    cond.split(|c| *c!='#')
        .filter(|arr|!arr.is_empty())
        .zip(records.iter())
        .all(|(a, b)| {
            a.len() == *b
        })
}

fn match_count(cond: Vec<char>, records: Vec<usize>) -> usize {
    let mut oper_count = 0;
    let mut dama_count = 0;
    let mut unkn_count = 0;

    let mut unkn_idx = Vec::new();

    for (i, c) in cond.iter().enumerate() {
        match *c {
            '?' => {
                unkn_idx.push(i);
                unkn_count += 1;
            },
            '#' => dama_count += 1,
            '.' => oper_count += 1,
            _ => unreachable!(),
        }
    }

    let sum: usize = records.iter().sum();
    let k = sum - dama_count;
    if k == 0 { return 1; }

    let combs = combination(unkn_idx.len(), k);
    let sum = combs.into_iter()
        .map(|comb|{
            let mut v = cond.clone();
            for i in comb.into_iter() {
                v[unkn_idx[i-1]] = '#';
            }
            if is_match(&v, &records) {1} else {0}
        })
        .sum();
    println!("match-count: {}", sum);

    sum

    // unkn_idx.combination(k)
    //     .map(|c|{
    //         let mut v = cond.clone();
    //         for i in c {
    //             v[*i] = '#';
    //         }
    //         if is_match(&v, &records) {1} else {0}
    //     })
    //     .sum()
}

fn part_1(input: &str) -> usize {
    input.split("\n")
        .map(|line|{
            let mut it = line.split(" ");
            let conditions = it.next().unwrap()
                .chars()
                .filter(|s| *s!=' ')
                .collect::<Vec<_>>();
            let records = it.next().unwrap()
                .split(",")
                .map(|s|s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            // println!("{:?}", conditions);
            // println!("{:?}", records);

            match_count(conditions, records)
        })
        .sum()
}


fn part_2(input: &str) -> usize {
    input.split("\n")
        .map(|line|{
            let mut it = line.split(" ");
            let mut conditions = it.next().unwrap()
                .chars()
                .filter(|s| *s!=' ')
                .collect::<Vec<_>>();
            conditions.push('?');
            conditions = conditions.repeat(5);
            conditions.pop();

            let records = it.next().unwrap()
                .split(",")
                .map(|s|s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .repeat(5);

            println!("{:?}", conditions);
            println!("{:?}", records);

            match_count(conditions, records) as usize
        })
        .sum()
}

fn combination(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut final_ret = Vec::new();
    let cur_vec = Vec::new();
    do_combination(&mut final_ret, cur_vec, n, k);

    final_ret
}

fn do_combination(final_ret: &mut Vec<Vec<usize>>, mut cur_vec: Vec<usize>, n: usize, k: usize) {
    if n < k { return; }
    if n == k {
        cur_vec.extend((1..=n).into_iter());
        final_ret.push(cur_vec);
        return;
    }
    if k == 1 {
        for i in 1..=n {
            let mut cloned = cur_vec.clone();
            cloned.push(i);
            final_ret.push(cloned);
        }
        return;
    }

    let mut vec_cloned = cur_vec.clone();
    vec_cloned.push(n);

    do_combination(final_ret, vec_cloned, n-1, k-1);
    do_combination(final_ret, cur_vec, n-1, k);
}

#[test]
fn solve() {
    let input = include_str!("input");
    let sum = part_1(input);
    println!("sum: {:?}", sum);
}

#[test]
fn test() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let sum = part_1(input);
    println!("sum = {}", sum);
    assert_eq!(sum, 21);

    let sum = part_2(input);
    println!("sum = {}", sum);
    assert_eq!(sum, 525152);
}

#[test]
fn test_combination() {
    let ret = combination(4,3);
    for v in ret.iter() {
        println!("{:?}", v);
    }
}