use std::collections::HashMap;

#[test]
fn solve() {
    let input = include_str!("input");
    let mut it = input.split("\n");
    let instructions = it.next().unwrap();

    let mut hash = HashMap::new();

    let mut start_nodes = Vec::new();

    it.filter(|s|!s.is_empty())
        .for_each(|node| {
            // GXT = (MQM, CHN)
            let mut it = node.split(" = ");
            let key = it.next().unwrap();
            if key.ends_with("A") {
                start_nodes.push(key);
            }
            let s = it.next().unwrap();
            let s = &s[1..s.len()-1];
            let mut it = s.split(", ");
            let left = it.next().unwrap();
            let right = it.next().unwrap();
            hash.insert(key, (left, right));
        });

    let mut steps = 0;
    let mut node = "AAA";
    for instruct in instructions.chars().cycle() {
        steps += 1;
        let v = hash.get(node).unwrap();

        node = match instruct {
            'L' => v.0,
            'R' => v.1,
            _ => unreachable!(),
        };

        if node == "ZZZ" {
            break;
        }
    }

    println!("steps: {}", steps);

    // ------- Part Two ----------
    // Calculate the LCM(Least Common Multiple) of the steps
    // https://www.calculatorsoup.com/calculators/math/lcm.php
    // 最小公倍数
    let steps = start_nodes.iter()
        .map(|node| {
            let mut steps = 0;
            let mut node = *node;
            for instruct in instructions.chars().cycle() {
                steps += 1;
                let v = hash.get(node).unwrap();

                node = match instruct {
                    'L' => v.0,
                    'R' => v.1,
                    _ => unreachable!(),
                };

                if node.ends_with("Z") {
                    break;
                }
            }
            steps
        })
        .fold(1_u128, |acc, x| {
            num::integer::lcm(acc, x)
        });

    println!("steps: {:?}", steps);

}
