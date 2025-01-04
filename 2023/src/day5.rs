use aoc_core::{blocks, Solution};

pub struct Day5;

impl Solution<'_> for Day5 {
    type Input = (Vec<u64>, SeedToLocation);
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &str) -> Self::Input {
        let mut iter = blocks(data);

        let (_, seeds) = iter
            .next()
            .unwrap()
            .split_once(':')
            .expect("Must contain colon");
        let seeds: Vec<u64> = seeds
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("Seeds are numbers"))
            .collect();

        let seed_to_soil = Map::parse(iter.next().unwrap());
        let soil_to_fertilizer = Map::parse(iter.next().unwrap());
        let fertilizer_to_water = Map::parse(iter.next().unwrap());
        let water_to_light = Map::parse(iter.next().unwrap());
        let light_to_temperature = Map::parse(iter.next().unwrap());
        let temperature_to_humidity = Map::parse(iter.next().unwrap());
        let humidity_to_location = Map::parse(iter.next().unwrap());

        let seed_to_location = ComposeMap {
            map1: seed_to_soil,
            map2: ComposeMap {
                map1: soil_to_fertilizer,
                map2: ComposeMap {
                    map1: fertilizer_to_water,
                    map2: ComposeMap {
                        map1: water_to_light,
                        map2: ComposeMap {
                            map1: light_to_temperature,
                            map2: ComposeMap {
                                map1: temperature_to_humidity,
                                map2: humidity_to_location,
                            },
                        },
                    },
                },
            },
        };
        (seeds, seed_to_location)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (seeds, seed_to_location) = input;
        seeds
            .into_iter()
            .map(|x| seed_to_location.map(x))
            .min()
            .unwrap()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let (seeds, seed_to_location) = input;
        // Cheating here by checking things in parallel. Probably there is a clever way
        // to do this instead of brute force, but this works fine on my machine (runtime ~ 1min).
        std::thread::scope(|s| {
            let threads = seeds
                .chunks_exact(2)
                .map(|x| match x {
                    [start, length] => s.spawn(|| {
                        let start = *start;
                        let end = start + *length;
                        (start..end).map(|x| seed_to_location.map(x)).min().unwrap()
                    }),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            threads
                .into_iter()
                .map(|t| t.join().unwrap())
                .min()
                .unwrap()
        })
    }
}

pub trait CanMap {
    fn map(&self, x: u64) -> u64;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapRange {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

impl MapRange {
    pub fn from_line(line: &str) -> Self {
        let mut iter = line
            .trim()
            .splitn(3, ' ')
            .map(|x| x.parse().expect("Must be numbers"));
        let dest_start = iter.next().unwrap();
        let source_start = iter.next().unwrap();
        let length = iter.next().unwrap();
        Self {
            dest_start,
            source_start,
            length,
        }
    }

    pub fn contains(&self, x: &u64) -> bool {
        (self.source_start..(self.source_start + self.length)).contains(x)
    }

    pub fn try_map(&self, x: u64) -> Option<u64> {
        if self.contains(&x) {
            let value = self.dest_start + (x - self.source_start);
            Some(value)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    pub fn parse(block: &str) -> Self {
        let ranges = block.lines().skip(1).map(MapRange::from_line).collect();
        Self { ranges }
    }
}

impl CanMap for Map {
    fn map(&self, x: u64) -> u64 {
        self.ranges.iter().find_map(|r| r.try_map(x)).unwrap_or(x)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposeMap<T, U> {
    map1: T,
    map2: U,
}

impl<T, U> CanMap for ComposeMap<T, U>
where
    T: CanMap,
    U: CanMap,
{
    fn map(&self, x: u64) -> u64 {
        let y = self.map1.map(x);
        self.map2.map(y)
    }
}

pub type SeedToLocation = ComposeMap<
    Map, // seed-to-soil
    ComposeMap<
        Map, // soil-to-fertilizer
        ComposeMap<
            Map, // fertilizer-to-water
            ComposeMap<
                Map, // water-to-light
                ComposeMap<
                    Map, // light-to-temperature
                    ComposeMap<
                        Map, // temperature-to-humidity
                        Map, // humidity-to-location
                    >,
                >,
            >,
        >,
    >,
>;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day5_example.txt");

    #[test]
    fn test_part1() {
        let input = Day5::parse_input(EXAMPLE_INPUT);
        let output = Day5::part_1(input);
        assert_eq!(output, 35);
    }

    #[test]
    fn test_part2() {
        let input = Day5::parse_input(EXAMPLE_INPUT);
        let output = Day5::part_2(input);
        assert_eq!(output, 46);
    }
}
