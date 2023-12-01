/*!
--- Day 1: Trebuchet?! ---
Something is wrong with global snow production, and you've been selected to take a look. 
The Elves have even given you a map; on it, they've used stars to mark the top fifty locations 
that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, 
you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; 
the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and 
where they're even sending you ("the sky") and why your map looks mostly 
blank ("you sure ask a lot of questions") and hang on did you just say the 
sky ("of course, where do you think snow comes from") when you realize that 
the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration 
document (your puzzle input) has been amended by a very young Elf who was apparently just excited to 
show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally 
contained a specific calibration value that the Elves now need to recover. On each line, 
the calibration value can be found by combining the first digit and the last digit (in that order) 
to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?


--- Part Two ---
Your calculation isn't quite right. It looks like some of the digits are actually spelled out 
with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. 
For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together 
produces 281.

What is the sum of all of the calibration values?

*/

pub fn solve() {
    let input = include_str!("input");
    let input = input.split("\n").collect::<Vec<_>>();
    // println!("input: {:#?}", input);

    let sum = input.iter()
        .fold(0, |acc, s| {
            let first = if let Some(c) = s.chars().find(|c|c.is_ascii_digit()) {
                c.to_digit(10).unwrap()
            } else { 0 };
            let last = if let Some(c) = s.chars().rev().find(|c|c.is_ascii_digit()) {
                c.to_digit(10).unwrap()
            } else { 0 };

            acc + first * 10 + last
        });

    println!("sum: {}", sum);

    let sum = input.iter()
        .fold(0, |acc, s| {
            acc + find_first_digit(s) * 10 + find_last_digit(s)
        });
    println!("sum: {}", sum);
}


use literal_enum::LiteralEnum;
use regex::Regex;

#[derive(LiteralEnum)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Digit {
    #[lit = "1"]
    D1,
    #[lit = "2"]
    D2,
    #[lit = "3"]
    D3,
    #[lit = "4"]
    D4,
    #[lit = "5"]
    D5,
    #[lit = "6"]
    D6,
    #[lit = "7"]
    D7,
    #[lit = "8"]
    D8,
    #[lit = "9"]
    D9,

    #[lit = "one"]
    One,
    #[lit = "two"]
    Two,
    #[lit = "three"]
    Three,
    #[lit = "four"]
    Four,
    #[lit = "five"]
    Five,
    #[lit = "six"]
    Six,
    #[lit = "seven"]
    Seven,
    #[lit = "eight"]
    Eight,
    #[lit = "nine"]
    Nine,
}

impl Digit {
    pub fn to_digit(&self) -> u32 {
        match self {
            Digit::D1 => 1,
            Digit::D2 => 2,
            Digit::D3 => 3,
            Digit::D4 => 4,
            Digit::D5 => 5,
            Digit::D6 => 6,
            Digit::D7 => 7,
            Digit::D8 => 8,
            Digit::D9 => 9,
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }
}

fn find_first_digit(s: &str) -> u32 {
    let reg = Regex::new("([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    if let Some(caps) = reg.captures(s) {
        let s = caps.get(0).unwrap().as_str();
        let d = Digit::try_from(s).unwrap();
        return d.to_digit();
    }
    0
}

fn find_last_digit(s: &str) -> u32 {
    let reg = Regex::new(".*([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    if let Some(caps) = reg.captures(s) {
        // println!("caps[0]: {}", caps.get(0).unwrap().as_str());
        let s = caps.get(1).unwrap().as_str();
        let d = Digit::try_from(s).unwrap();
        return d.to_digit();
    }
    0
}

#[test]
fn test() {
    solve();
}