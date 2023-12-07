use std::{collections::HashMap, cmp::Ordering, str::FromStr};


#[derive(Debug)]
pub struct Hand {
    pub cards: [char; 5],
    pub bid: u64,
}

impl Hand {
    pub fn new(cards: [char; 5], bid: u64) -> Self {
        Self { cards, bid }
    }

    pub fn to_score(&self) -> [char; 6] {
        let mut score = ['0'; 6];
        let mut cards = self.cards;

        let mut hash = HashMap::new();
        for c in cards.iter() {
            if let Some(v) = hash.get(c) {
                hash.insert(*c, v + 1);
            } else {
                hash.insert(*c, 1);
            }
        }

        // 降序
        cards.sort_by(|a, b| {
            // 优先以数量排序
            let order = hash.get(b).unwrap().cmp(&hash.get(a).unwrap());
            if order != Ordering::Equal {
                order
            } else {
                char_to_score(*b).cmp(&char_to_score(*a))
            }
        });

        // Five of a kind
        if hash.len() == 1 {
            score[0] = '7';
        }

        if hash.len() == 2 {
            score[0] = if cards[2] == cards[3] {
                // Four of a kind: AAAAB
                '6'
            } else {
                // Full house: AAABB
                '5'
            };
        }

        if hash.len() == 3 {
            score[0] = if cards[1] == cards[2] {
                // AAABC
                '4'
            } else {
                // AABBC
                '3'
            };
        }
        if hash.len() == 4 {
            // AABCD
            score[0] = '2';
        }
        if hash.len() == 5 {
            // ABCDE
            score[0] = '1';
        }

        score[1] = char_to_score(self.cards[0]);
        score[2] = char_to_score(self.cards[1]);
        score[3] = char_to_score(self.cards[2]);
        score[4] = char_to_score(self.cards[3]);
        score[5] = char_to_score(self.cards[4]);

        score
    }

    /// Part Two
    pub fn to_score_with_joker(&self) -> [char; 6] {
        let mut score = ['0'; 6];
        let mut cards_without_j: Vec<char> = self.cards.iter().filter(|c| **c != 'J').map(|c|*c).collect();

        score[1] = char_to_score_with_joker(self.cards[0]);
        score[2] = char_to_score_with_joker(self.cards[1]);
        score[3] = char_to_score_with_joker(self.cards[2]);
        score[4] = char_to_score_with_joker(self.cards[3]);
        score[5] = char_to_score_with_joker(self.cards[4]);

        let mut hash = HashMap::new();
        for c in self.cards.iter() {
            if let Some(v) = hash.get(c) {
                hash.insert(*c, v + 1);
            } else {
                hash.insert(*c, 1);
            }
        }

        let j_count = *hash.get(&'J').unwrap_or(&0);

        // 降序
        cards_without_j.sort_by(|a, b| {
            // 优先以数量排序
            let order = hash.get(b).unwrap().cmp(&hash.get(a).unwrap());
            if order != Ordering::Equal {
                order
            } else {
                char_to_score_with_joker(*b).cmp(&char_to_score_with_joker(*a))
            }
        });

        if cards_without_j.is_empty() {
            // Five of 'J's
            cards_without_j = vec!['J', 'J', 'J', 'J', 'J'];
        } else {
            hash.remove(&'J');
            for _ in 0.. j_count {
                let count = *hash.get(&cards_without_j[0]).unwrap();
                hash.insert(cards_without_j[0], count + 1);
                cards_without_j.insert(0, cards_without_j[0]);
            }
        }

        let cards = cards_without_j;

        score[0] = match hash.len() {
            // Five of a kind: AAAAA
            1 => '7',
            // AAAAB or AAABB
            2 => {
                // Four of a kind AAAAB
                if cards[2] == cards[3] {
                    '6'
                } else {
                    // Full house: AAABB
                    '5'
                }
            },
            // AAABC or AABBC
            3 => {
                if cards[1] == cards[2] {
                    // Three of a kind: AAABC
                    '4'
                } else {
                    // Two pair: AABBC
                    '3'
                }
            },
            // AABCD
            4 => {
                // One pair: AABCD
                '2'
            },
            // High card: ABCDE
            5 => { '1' },
            _ => unreachable!(),
        };

        score
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut it = line.split(" ");
        let s = it.next().unwrap();
        let bid: u64 = it.next().unwrap().parse().unwrap();

        let mut it = s.chars();
        let mut cards = ['0'; 5];
        for c in cards.iter_mut() {
            *c = it.next().unwrap();
        }

        // println!("cards: {:?}, bid: {}", cards, bid);

        Ok(Hand::new(cards, bid))
    }
}

fn char_to_score(c: char) -> char {
    match c {
        'T' => 'A',
        'J' => 'B',
        'Q' => 'C',
        'K' => 'D',
        'A' => 'E',
        x => x,
    }
}

fn char_to_score_with_joker(c: char) -> char {
    match c {
        'J' => '1',
        'T' => 'A',
        'Q' => 'B',
        'K' => 'C',
        'A' => 'D',
        _ => c,
    }
}


#[test]
fn solve() {
    let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    let input = include_str!("input");

    let mut vec = input.split("\n")
        .map(|line| {
            line.parse::<Hand>().unwrap()
        })
        .collect::<Vec<_>>();

    vec.sort_by_key(|h| h.to_score());
    let sum = vec.iter()
        .enumerate()
        .fold(0, |acc, (i, v)| {
            let rank = i + 1;
            let rank = rank as u64;
            acc + (rank * v.bid)
        });

    println!("sum: {}", sum);


    // Part Two
    // J is Joker that can act like whatever card
    vec.sort_by_key(|h|h.to_score_with_joker());
    let sum = vec.iter()
        .enumerate()
        .fold(0, |acc, (i, v)| {
            let rank = i + 1;
            let rank = rank as u64;
            acc + (rank * v.bid)
        });

    println!("sum: {}", sum);
}