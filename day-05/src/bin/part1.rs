use std::{collections::VecDeque, ops::Range, usize};

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 1 answer: {answer}");
}

fn process(input: &str) -> usize {
    let mut lines: VecDeque<&str> = input.lines().map(|l| l.trim()).collect();

    // remove any empty lines at the beginning
    while let Some(line) = lines.front() {
        if line.starts_with("seeds:") {
            break;
        }
        lines.pop_front();
    }

    // seeds
    let seeds: Vec<usize> = lines
        .pop_front()
        .expect("No seeds found")
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|num| num.parse().expect("failed to parse a number"))
        .collect();

    // skip empty line
    lines.pop_front();

    // seed-to-soil map
    let seed_to_soil = parse_map(&mut lines);

    // soil-to-fertilizer
    let soil_to_fertilizer = parse_map(&mut lines);

    // fertilizer-to-water
    let fertilizer_to_water = parse_map(&mut lines);

    // water-to-light
    let water_to_light = parse_map(&mut lines);

    // light-to-temperature
    let light_to_temperature = parse_map(&mut lines);

    // temperature-to-humidity
    let temperature_to_humidity = parse_map(&mut lines);

    // humidity-to-location
    let humidity_to_location = parse_map(&mut lines);

    // find nearest location: map all seeds to locations and find min
    seeds
        .into_iter()
        .map(|seed| {
            let s = seed_to_soil.get(seed);
            let f = soil_to_fertilizer.get(s);
            let w = fertilizer_to_water.get(f);
            let l = water_to_light.get(w);
            let t = light_to_temperature.get(l);
            let h = temperature_to_humidity.get(t);
            humidity_to_location.get(h)
        })
        .min()
        .expect("could not find min location")
}

#[derive(Default, Debug)]
struct SourceDestinationMap {
    ranges: Vec<(Range<usize>, Range<usize>)>,
}

impl SourceDestinationMap {
    // looks up a value in the destination range using the value from the source range
    fn get(&self, idx: usize) -> usize {
        for (source, destination) in self.ranges.iter() {
            if source.contains(&idx) {
                let distance_from_start = idx - source.start;
                return destination.start + distance_from_start;
            }
        }
        idx
    }
}

fn parse_map(lines: &mut VecDeque<&str>) -> SourceDestinationMap {
    let mut map = SourceDestinationMap::default();
    while let Some(row) = lines.pop_front() {
        if row.is_empty() {
            break;
        }

        if row.ends_with("map:") {
            continue;
        }

        map.ranges.push(parse_row(row));
    }
    map.ranges.sort_by_key(|(src, _)| src.start);
    map
}

fn parse_row(row: &str) -> (Range<usize>, Range<usize>) {
    let nums: Vec<usize> = row
        .split(' ')
        .map(|num| num.parse().expect("failed to parse a number"))
        .collect();

    if nums.len() != 3 {
        panic!("Expected three numbers in row: {row}");
    }

    let dst_start = nums[0];
    let src_start = nums[1];
    let length = nums[2];
    (
        src_start..(src_start + length),
        dst_start..(dst_start + length),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_process() {
        let input = r#"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "#;
        assert_eq!(35, process(input));
    }
}
