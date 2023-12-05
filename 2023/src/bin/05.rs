use itertools::Itertools;
use std::cmp::min;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    mappings: Vec<Map>,
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut sections = value.split("\n\n");
        let seeds = sections
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let maps = sections.map(Map::from).collect();
        Almanac {
            seeds,
            mappings: maps,
        }
    }
}

impl Almanac {
    pub fn apply(&self, seed: u64) -> u64 {
        let mut current = seed;
        self.mappings.iter().for_each(|map| {
            current = map.apply(current);
        });
        current
    }

    pub fn apply_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut current = ranges;
        self.mappings.iter().for_each(|map| {
            current = map.apply_map(&current);
        });
        current
    }
}

#[derive(Debug)]
struct Map {
    mapping: Vec<Mapping>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut parsed_section: Vec<Mapping> = value.lines().skip(1).map(Mapping::from).collect();
        parsed_section.sort_by(|a, b| a.range.start.cmp(&b.range.start));
        Map {
            mapping: parsed_section,
        }
    }
}

impl Map {
    pub fn apply(&self, value: u64) -> u64 {
        let maybe_mapping = self.mapping.iter().find(|m| m.range.contains(value));
        if let Some(mapping) = maybe_mapping {
            mapping.apply(value).unwrap()
        } else {
            value
        }
    }

    pub fn apply_map(&self, ranges: &Vec<Range>) -> Vec<Range> {
        let mut output: Vec<Range> = Vec::new();
        for range in ranges {
            let mut start = range.start;
            self.mapping
                .iter()
                .filter(|m| m.range.overlaps(range))
                .for_each(|mapping| {
                    // no mapping applied
                    if start < mapping.range.start {
                        output.push(Range {
                            start,
                            end: mapping.range.start - 1,
                        });
                        start = mapping.range.start;
                    }
                    // mapping applied
                    let end = min(mapping.range.end, range.end);
                    let mapped_start = mapping.apply(start).unwrap();
                    let mapped_end = mapping.apply(end).unwrap();
                    output.push(Range {
                        start: mapped_start,
                        end: mapped_end,
                    });
                    start = end;
                });
            // final no mapping
            if start < range.end {
                output.push(Range {
                    start,
                    end: range.end,
                });
            }
        }
        output
    }
}

#[derive(Debug)]
struct Mapping {
    range: Range,
    diff: u64,
    negative: bool,
}

impl From<&str> for Mapping {
    fn from(value: &str) -> Self {
        let (destination_start, source_start, range) = value
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        let negative = destination_start < source_start;
        let diff = if negative {
            source_start - destination_start
        } else {
            destination_start - source_start
        };
        Mapping {
            range: Range {
                start: source_start,
                end: source_start + range - 1,
            },
            diff,
            negative,
        }
    }
}

impl Mapping {
    pub fn apply(&self, value: u64) -> Option<u64> {
        if !self.range.contains(value) {
            return None;
        }
        if self.negative {
            Some(value - self.diff)
        } else {
            Some(value + self.diff)
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = Almanac::from(input);
    let location = almanac.seeds.iter().map(|seed| almanac.apply(*seed)).min();
    location
}

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn contains(&self, value: u64) -> bool {
        value >= self.start && value <= self.end
    }
    pub fn overlaps(&self, other: &Range) -> bool {
        let overlaps_start = self.start <= other.start && self.end >= other.start;
        let overlaps_end = self.start <= other.end && self.end >= other.end;
        let contained = self.start >= other.start && self.end <= other.end;
        overlaps_start || overlaps_end || contained
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac = Almanac::from(input);
    let mut iter = almanac.seeds.iter();
    let mut ranges: Vec<Range> = Vec::new();
    while let Some(one) = iter.next() {
        if let Some(two) = iter.next() {
            ranges.push(Range {
                start: *one,
                end: *one + *two - 1,
            });
        }
    }
    let output = almanac.apply_ranges(ranges).iter().map(|x| x.start).min();
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_overlap_range() {
        //overlap at range start
        assert!(Range { start: 5, end: 5 }.overlaps(&Range { start: 0, end: 5 }));
        //overlap at range end
        assert!(Range { start: 0, end: 4 }.overlaps(&Range { start: 0, end: 5 }));
        //overlap inside range
        assert!(Range { start: 0, end: 4 }.overlaps(&Range { start: 2, end: 3 }));
        // no overlap after
        assert!(!Range { start: 0, end: 4 }.overlaps(&Range { start: 6, end: 7 }));
        // no overlap before
        assert!(!Range { start: 10, end: 14 }.overlaps(&Range { start: 6, end: 7 }));
    }
}
