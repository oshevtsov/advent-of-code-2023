use anyhow::{anyhow, bail, Error, Result};
use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn process(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|l| match l.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed),
            _ => None,
        })
        .map(process_row)
        .sum()
}

fn process_row(row: &str) -> u32 {
    match str::parse::<Game>(row) {
        Ok(game) => {
            if game
                .cube_sets
                .iter()
                .all(|cs| cs.red <= RED && cs.green <= GREEN && cs.blue <= BLUE)
            {
                return game.id;
            }
            0
        }
        Err(err) => panic!("{err}"),
    }
}

struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

#[derive(Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for CubeSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cube_set = CubeSet::default();
        for cube in s.split(',').filter_map(|c| match c.trim() {
            trimmed if !trimmed.is_empty() => Some(trimmed),
            _ => None,
        }) {
            if let Some((amount_str, color)) = cube.split_once(' ') {
                let amount = amount_str.parse()?;
                match color {
                    "red" => cube_set.red = amount,
                    "green" => cube_set.green = amount,
                    "blue" => cube_set.blue = amount,
                    other => bail!("Unrecognized color: {other}"),
                }
            } else {
                bail!("Failed to parse {cube}, expected format: '<number> <color>'");
            }
        }
        Ok(cube_set)
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((game_id, cubes)) = s.split_once(':') {
            if let Some((_, id_str)) = game_id.split_once(' ') {
                let id = id_str.parse()?;
                let cube_sets: Vec<CubeSet> = cubes
                    .split(';')
                    .filter_map(|c| match c.trim() {
                        trimmed if !trimmed.is_empty() => c.parse().ok(),
                        _ => None,
                    })
                    .collect();

                return Ok(Game { id, cube_sets });
            }
            bail!("Failed to parse Game id from '{game_id}'");
        }
        Err(anyhow!("Failed to parse a Game from: {s}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_row_all_sets_ok_one_digit_id() {
        let row = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(1, process_row(row));
    }

    #[test]
    fn test_process_row_all_sets_ok_multi_digit_id() {
        let row = "Game 99: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(99, process_row(row));
    }

    #[test]
    fn test_process_row_some_sets_not_ok() {
        let row = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(0, process_row(row));
    }

    #[test]
    fn test_process() {
        let input = r#"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        assert_eq!(8, process(input));
    }
}
