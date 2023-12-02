/*!
--- Day 2: Cube Conundrum ---
You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.

You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?

--- Part Two ---
The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!

As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?

Again consider the example games from earlier:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
Game 4 required at least 14 red, 3 green, and 15 blue cubes.
Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.

For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?
*/

use std::str::FromStr;


#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub info: Vec<GameInfo>
}

#[derive(Debug)]
pub struct GameInfo {
    red: u32,
    blue: u32,
    green: u32,
}

/// 12 red, 2 green, 5 blue
impl FromStr for GameInfo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self { red: 0, blue: 0, green: 0 };
        for color in s.split(",") {
            let mut it = color.trim().split(" ");
            let num: u32 = it.next().unwrap().parse().unwrap();
            let c = it.next().unwrap();
            match c {
                "red" => this.red = num,
                "green" => this.green = num,
                "blue" => this.blue = num,
                _ => unreachable!(),
            }
        }
        Ok(this)
    }
}

#[test]
fn solve() {
    let input = include_str!("input");
    let games = input.split("\n")
        .map(|s|{
            let mut it = s.split(":");

            let id = it.next().unwrap();

            // Game 1
            let id: u32 = id[5..].parse().unwrap();

            let v = it.next().unwrap();
            let vec: Vec<_> = v.split(";")
                .map(|s|{
                    let game_info: GameInfo = s.parse().unwrap();
                    game_info
                })
                .collect();

            Game {
                id,
                info: vec,
            }
        })
        .collect::<Vec<_>>();

    println!("games: {:?}", games);

    // Part One
    // 12 red, 13 green, 14 blue
    let red = 12;
    let green = 13;
    let blue = 14;

    let sum: u32 = games.iter()
        .filter(|game| {
            game.info
                .iter()
                .all(|info| {
                    info.red <= red
                    && info.green <= green
                    && info.blue <= blue
                })
        })
        .map(|game|{
            game.id
        })
        .sum();

    println!("sum: {}", sum);

    // Part Two
    let sum: u32 = games.iter()
        .map(|game|{
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            game.info
                .iter()
                .for_each(|info|{
                    if info.red > red {
                        red = info.red;
                    }
                    if info.green > green {
                        green = info.green;
                    }
                    if info.blue > blue {
                        blue = info.blue;
                    }
                });
            red * green * blue
        })
        .sum();

    println!("sum: {}", sum);

}
