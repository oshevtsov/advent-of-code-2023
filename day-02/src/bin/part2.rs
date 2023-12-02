use anyhow::{anyhow, bail, Error, Result};
use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 2 answer: {answer}");
}

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
        Ok(game) => game.max_red() * game.max_green() * game.max_blue(),
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

impl Game {
    fn max_red(&self) -> u32 {
        match self.cube_sets.iter().max_by_key(|s| s.red) {
            Some(cs) => cs.red,
            None => 0,
        }
    }

    fn max_green(&self) -> u32 {
        match self.cube_sets.iter().max_by_key(|s| s.green) {
            Some(cs) => cs.green,
            None => 0,
        }
    }

    fn max_blue(&self) -> u32 {
        match self.cube_sets.iter().max_by_key(|s| s.blue) {
            Some(cs) => cs.blue,
            None => 0,
        }
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
            bail!("Failed to parse Game id");
        }
        Err(anyhow!("Failed to parse a Game"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_process() {
        let input = r#"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        assert_eq!(2286, process(input));
    }
}
