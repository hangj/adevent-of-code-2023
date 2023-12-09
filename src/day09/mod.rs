
fn generate_next_number(arr: Vec<i64>) -> (i64, i64) {

    let mut cur_arr = arr;
    let mut sum = *cur_arr.last().unwrap();
    let mut new_first = *cur_arr.first().unwrap();

    let mut sign = -1;

    let mut additional_sequence = vec![];

    loop {
        if cur_arr.iter().all(|v|*v==0) { break; }

        additional_sequence.clear();
        for i in 0..cur_arr.len()-1 {
            let diff = cur_arr[i+1] - cur_arr[i];
            additional_sequence.push(diff);
        }

        std::mem::swap(&mut cur_arr, &mut additional_sequence);

        sum += *cur_arr.last().unwrap();

        new_first += *cur_arr.first().unwrap() * sign;
        sign = -sign;
    }

    (new_first, sum)
}

#[test]
fn solve() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    let input = include_str!("input");

    let sum = input.split("\n")
        .map(|line| {
            let nums = line.split(" ")
                .filter(|s|!s.is_empty())
                .map(|s|s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            generate_next_number(nums)
        })
        .fold((0, 0), |acc, v| {
            (acc.0 + v.0, acc.1 + v.1)
        });
    println!("sum: {:?}", sum);
}



