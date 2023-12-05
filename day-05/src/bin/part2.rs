use std::{collections::VecDeque, ops::Range, usize};

fn main() {
    let input = include_str!("./input.txt");
    let answer = process(input);
    println!("Part 2 answer: {answer}");
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
    let mut seed_ranges: Vec<Range<usize>> = lines
        .pop_front()
        .expect("No seeds found")
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|num| num.parse().expect("failed to parse a number"))
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|c| c[0]..(c[0] + c[1]))
        .collect();
    seed_ranges.sort_by_key(|r| r.start);

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

    // find nearest location: map all source (aka seed) ranges onto destination ranges, until we
    // find all the location ranges
    let soil_ranges = seed_to_soil.destination_ranges(&seed_ranges);
    let fertilizer_ranges = soil_to_fertilizer.destination_ranges(&soil_ranges);
    let water_ranges = fertilizer_to_water.destination_ranges(&fertilizer_ranges);
    let light_ranges = water_to_light.destination_ranges(&water_ranges);
    let temperature_ranges = light_to_temperature.destination_ranges(&light_ranges);
    let humidity_ranges = temperature_to_humidity.destination_ranges(&temperature_ranges);
    let location_ranges = humidity_to_location.destination_ranges(&humidity_ranges);

    // since location ranges are sorted, the nearest location is at the start of the first range
    location_ranges
        .first()
        .expect("location range cannot be empty")
        .start
}

// split the first range using the other into three parts:
// - before: all numbers that are on the left side of `other`
// - common: all numbers that are common between the input ranges
// - after: all numbers that are on the right side of `other`
fn split_by(r: &Range<usize>, other: &Range<usize>) -> RangeSplit {
    let mut before: Option<Range<usize>> = None;
    let mut common: Option<Range<usize>> = None;
    let mut after: Option<Range<usize>> = None;

    if r.start < other.start {
        before = if r.end < other.start {
            Some(r.clone())
        } else {
            Some(r.start..other.start)
        };
    }

    if r.end > other.end {
        after = if r.start > other.end {
            Some(r.clone())
        } else {
            Some(other.end..r.end)
        };
    }

    if r.start <= other.end && r.end >= other.start {
        let common_start = std::cmp::max(r.start, other.start);
        let common_end = std::cmp::min(r.end, other.end);
        common = Some(common_start..common_end);
    }
    RangeSplit::new(before, common, after)
}

// represents the result of splitting a range using another range as pivot
#[derive(Default, Debug)]
struct RangeSplit {
    before: Option<Range<usize>>,
    common: Option<Range<usize>>,
    after: Option<Range<usize>>,
}

impl RangeSplit {
    fn new(
        before: Option<Range<usize>>,
        common: Option<Range<usize>>,
        after: Option<Range<usize>>,
    ) -> Self {
        Self {
            before,
            common,
            after,
        }
    }
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

    // maps all source ranges to the corresponding non-overlapping destination ranges
    fn destination_ranges(&self, source_ranges: &[Range<usize>]) -> Vec<Range<usize>> {
        let mut destination_ranges: Vec<Range<usize>> = Vec::new();
        for source_range in source_ranges.iter() {
            let mut source_range_remainder = Some(source_range.clone());
            for (mapped_source_range, _) in self.ranges.iter() {
                // if there current source range is exhausted, move to the next
                if source_range_remainder.is_none() {
                    break;
                }

                // split the remainder range into non-overlapping pieces based on the mapped source range
                let remainder_range = source_range_remainder.unwrap();
                let range_split = split_by(&remainder_range, mapped_source_range);

                // before piece is not mapped and therefore maps to itself (note that all vectors
                // of ranges are sorted)
                if let Some(r) = range_split.before {
                    destination_ranges.push(r);
                }

                // common piece should be mapped onto the destination range
                if let Some(r) = range_split.common {
                    let start = self.get(r.start);
                    let end = self.get(r.end - 1) + 1;
                    destination_ranges.push(start..end);
                }

                // after piece should be kept for comparison with other mapped source ranges
                source_range_remainder = range_split.after;
            }

            // if we have looked at all mapped source ranges and there is still some remainder, it
            // means it was not in any of the mapped source ranges and therefore maps onto itself
            if let Some(r) = source_range_remainder {
                let start = self.get(r.start);
                let end = self.get(r.end - 1) + 1;
                destination_ranges.push(start..end);
            }
        }
        destination_ranges.sort_by_key(|r| r.start);
        destination_ranges
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
    fn part2_process() {
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
        assert_eq!(46, process(input));
    }
}
