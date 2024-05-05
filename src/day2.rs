use std::{fs, str::FromStr};

struct Game {
    subsets: Vec<GameSubset>,
    game_id: u32,
}
struct GameSubset(Vec<CubeColor>);

#[derive(Debug)]
enum ParseInputErr {
    CubeColor,
}

enum CubeColor {
    Blue(u16),
    Green(u16),
    Red(u16),
}
impl CubeColor {
    fn is_possible(&self) -> bool {
        match *self {
            CubeColor::Blue(blue) => blue <= 14,
            CubeColor::Green(green) => green <= 13,
            CubeColor::Red(red) => red <= 12,
        }
    }
}
impl FromStr for CubeColor {
    type Err = ParseInputErr;

    // example input: "3 green"
    fn from_str(cube_str: &str) -> Result<Self, Self::Err> {
        let mut cube_str = cube_str.split_whitespace();

        let quantity: u16 = cube_str
            .next()
            .unwrap()
            .trim()
            .parse()
            .expect("Error converting quantity");
        let color = cube_str.next().unwrap().trim().to_lowercase();

        match (&color[0..], quantity) {
            ("blue", quantity) => Ok(CubeColor::Blue(quantity)),
            ("red", quantity) => Ok(CubeColor::Red(quantity)),
            ("green", quantity) => Ok(CubeColor::Green(quantity)),
            _ => Err(Self::Err::CubeColor),
        }
    }
}

impl FromStr for GameSubset {
    type Err = ParseInputErr;

    //input example "3 green, 3 blue, 6 red"
    fn from_str(subset: &str) -> Result<Self, Self::Err> {
        let game_subset = subset
            .split(',')
            .map(|cubes| match CubeColor::from_str(cubes) {
                Ok(parsed_cube) => parsed_cube,
                Err(e) => panic!("{e:?}"),
            })
            .collect::<Vec<CubeColor>>();

        Ok(GameSubset(game_subset))
    }
}

impl Game {
    fn new(game_str: &str) -> Self {
        let mut game_str = game_str.split(':');

        let game_id: u32 = game_str
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let game_subsets = game_str
            .next()
            .unwrap()
            .split(';')
            .map(|subset_str| GameSubset::from_str(subset_str).unwrap())
            .collect();

        Game {
            subsets: game_subsets,
            game_id,
        }
    }

    fn is_possible(&self) -> bool {
        self.subsets
            .iter()
            .all(|GameSubset(cubes)| cubes.iter().all(|cube_color| cube_color.is_possible()))
    }
    fn powers_product(&self) -> u32 {
        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;

        for subset in &self.subsets {
            for cube_color in subset.0.iter() {
                match *cube_color {
                    CubeColor::Blue(val) if val > max_blue => max_blue = val,
                    CubeColor::Green(val) if val > max_green => max_green = val,
                    CubeColor::Red(val) if val > max_red => max_red = val,
                    _ => (),
                }
            }
        }

        [max_blue as u32, max_green as u32, max_red as u32]
            .into_iter()
            .reduce(|f: u32, s: u32| f * s)
            .unwrap()
    }
}

pub fn part1() {
    println!("\tPart 1");
    let input = fs::read_to_string("input.txt").unwrap();

    let possible_counts: u32 = input
        .lines()
        .map(|game_str| Game::new(game_str))
        .filter(|game| game.is_possible())
        .map(|game| game.game_id)
        .sum();

    println!("\t\tIds sum of possible games: {possible_counts}")
}

pub fn part2() {
    println!("\tPart 2");
    let input = fs::read_to_string("input.txt").unwrap();

    let sum_of_power_of_sets: u32 = input
        .lines()
        .map(|game_str| Game::new(game_str).powers_product())
        .sum();

    println!("\t\tPowers sum of games: {sum_of_power_of_sets}")
}
