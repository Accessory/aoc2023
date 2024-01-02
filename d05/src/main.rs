#![feature(array_chunks)]
#![feature(iter_array_chunks)]
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

use utils::get_input_path;

enum ParsingState {
    Seeds,
    SeedToSoil,
    SoilToFertilizerMap,
    FertilizerToWaterMap,
    WaterToLightMap,
    LightToTemperatureMap,
    TemperatureToHumidityMap,
    HumidityToLocationMap,
}

#[derive(Debug, Clone, Copy)]
struct CustomRange {
    start: usize,
    length: usize,
}
impl CustomRange {
    fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }
}

#[derive(Default, Debug)]
struct RangeMap {
    pub destination_start: usize,
    pub source_start: usize,
    pub length: usize,
}

impl RangeMap {
    fn get_source_range(&self) -> Range<usize> {
        self.source_start..self.source_start + self.length
    }
}

impl From<String> for RangeMap {
    fn from(value: String) -> Self {
        let mut split = value.split(' ');
        Self {
            destination_start: split.next().unwrap().parse().unwrap(),
            source_start: split.next().unwrap().parse().unwrap(),
            length: split.next().unwrap().parse().unwrap(),
        }
    }
}

trait RangeMapUtils {
    fn convert_location(&self, value: usize) -> usize;
    fn next_ranges(&self, ranges: Vec<CustomRange>) -> Vec<CustomRange>;
}

impl RangeMapUtils for Vec<RangeMap> {
    fn convert_location(&self, value: usize) -> usize {
        for rm in self.iter() {
            if rm.get_source_range().contains(&value) {
                return rm.destination_start + (value - rm.source_start);
            }
        }
        value
    }

    fn next_ranges(&self, ranges: Vec<CustomRange>) -> Vec<CustomRange> {
        let mut rtn = Vec::new();
        for mut range in ranges {
            'l1: loop {
                for range_map in self {
                    let mut range_map_length = range_map.length;
                    if range_map.source_start <= range.start
                        && range.start < range_map.source_start + range_map_length
                    {
                        range_map_length -= max(
                            range.start as i64 - range_map.source_start as i64,
                            range_map_length as i64 - range.length as i64,
                        ) as usize;

                        rtn.push(CustomRange::new(
                            range.start - range_map.source_start + range_map.destination_start,
                            range_map_length,
                        ));

                        range.start += range_map_length;
                        range.length -= range_map_length;
                        if range.length == 0 {
                            break 'l1;
                        }
                        continue 'l1;
                    }
                }

                rtn.push(range);
                break 'l1;
            }
        }
        rtn
    }
}

#[derive(Debug, Default)]
struct Context {
    pub seeds: Vec<usize>,
    pub seed_to_soil: Vec<RangeMap>,
    pub soil_to_fertilizer_map: Vec<RangeMap>,
    pub fertilizer_to_water_map: Vec<RangeMap>,
    pub water_to_light_map: Vec<RangeMap>,
    pub light_to_temperature_map: Vec<RangeMap>,
    pub temperature_to_humidity_map: Vec<RangeMap>,
    pub humidity_to_location_map: Vec<RangeMap>,
}

fn run(input_file: &str) {
    // Preamble
    let mut parsing_state = ParsingState::Seeds;
    let mut context = Context::default();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if line.is_empty() {
            continue;
        }

        match &line[0..5] {
            "seeds" => {
                context.seeds = line[7..]
                    .split(' ')
                    .into_iter()
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect();
                continue;
            }
            "seed-" => {
                parsing_state = ParsingState::SeedToSoil;
                continue;
            }
            "soil-" => {
                parsing_state = ParsingState::SoilToFertilizerMap;
                continue;
            }
            "ferti" => {
                parsing_state = ParsingState::FertilizerToWaterMap;
                continue;
            }
            "water" => {
                parsing_state = ParsingState::WaterToLightMap;
                continue;
            }
            "light" => {
                parsing_state = ParsingState::LightToTemperatureMap;
                continue;
            }
            "tempe" => {
                parsing_state = ParsingState::TemperatureToHumidityMap;
                continue;
            }
            "humid" => {
                parsing_state = ParsingState::HumidityToLocationMap;
                continue;
            }
            _ => {}
        };

        match parsing_state {
            ParsingState::Seeds => panic!("Should not be here!"),
            ParsingState::SeedToSoil => context.seed_to_soil.push(line.into()),
            ParsingState::SoilToFertilizerMap => context.soil_to_fertilizer_map.push(line.into()),
            ParsingState::FertilizerToWaterMap => context.fertilizer_to_water_map.push(line.into()),
            ParsingState::WaterToLightMap => context.water_to_light_map.push(line.into()),
            ParsingState::LightToTemperatureMap => {
                context.light_to_temperature_map.push(line.into())
            }
            ParsingState::TemperatureToHumidityMap => {
                context.temperature_to_humidity_map.push(line.into())
            }
            ParsingState::HumidityToLocationMap => {
                context.humidity_to_location_map.push(line.into())
            }
        };
    }

    // Solve
    let mut location_numbers = Vec::new();
    for mut seed in context.seeds {
        // Seed to Soil
        seed = context.seed_to_soil.convert_location(seed);
        // Soil to Fertilizer
        seed = context.soil_to_fertilizer_map.convert_location(seed);
        // Fertilizer to Water
        seed = context.fertilizer_to_water_map.convert_location(seed);
        // Water to Light
        seed = context.water_to_light_map.convert_location(seed);
        // Light to Temperature
        seed = context.light_to_temperature_map.convert_location(seed);
        // Temperature to Humidity
        seed = context.temperature_to_humidity_map.convert_location(seed);
        // Humidity to Location
        seed = context.humidity_to_location_map.convert_location(seed);
        location_numbers.push(seed);
    }

    // Result
    let result = location_numbers.iter().min().unwrap();

    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut parsing_state = ParsingState::Seeds;
    let mut context = Context::default();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if line.is_empty() {
            continue;
        }

        match &line[0..5] {
            "seeds" => {
                context.seeds = line[7..]
                    .split(' ')
                    .into_iter()
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect();
                continue;
            }
            "seed-" => {
                parsing_state = ParsingState::SeedToSoil;
                continue;
            }
            "soil-" => {
                parsing_state = ParsingState::SoilToFertilizerMap;
                continue;
            }
            "ferti" => {
                parsing_state = ParsingState::FertilizerToWaterMap;
                continue;
            }
            "water" => {
                parsing_state = ParsingState::WaterToLightMap;
                continue;
            }
            "light" => {
                parsing_state = ParsingState::LightToTemperatureMap;
                continue;
            }
            "tempe" => {
                parsing_state = ParsingState::TemperatureToHumidityMap;
                continue;
            }
            "humid" => {
                parsing_state = ParsingState::HumidityToLocationMap;
                continue;
            }
            _ => {}
        };

        match parsing_state {
            ParsingState::Seeds => panic!("Should not be here!"),
            ParsingState::SeedToSoil => context.seed_to_soil.push(line.into()),
            ParsingState::SoilToFertilizerMap => context.soil_to_fertilizer_map.push(line.into()),
            ParsingState::FertilizerToWaterMap => context.fertilizer_to_water_map.push(line.into()),
            ParsingState::WaterToLightMap => context.water_to_light_map.push(line.into()),
            ParsingState::LightToTemperatureMap => {
                context.light_to_temperature_map.push(line.into())
            }
            ParsingState::TemperatureToHumidityMap => {
                context.temperature_to_humidity_map.push(line.into())
            }
            ParsingState::HumidityToLocationMap => {
                context.humidity_to_location_map.push(line.into())
            }
        };
    }

    let mut result = usize::MAX;
    for [seed_start, seed_length] in context.seeds.array_chunks() {
        let mut ranges = vec![CustomRange::new(*seed_start, *seed_length)];

        // Seed to Soil
        ranges = context.seed_to_soil.next_ranges(ranges);
        // Soil to Fertilizer
        ranges = context.soil_to_fertilizer_map.next_ranges(ranges);
        // Fertilizer to Water
        ranges = context.fertilizer_to_water_map.next_ranges(ranges);
        // Water to Light
        ranges = context.water_to_light_map.next_ranges(ranges);
        // Light to Temperature
        ranges = context.light_to_temperature_map.next_ranges(ranges);
        // Temperature to Humidity
        ranges = context.temperature_to_humidity_map.next_ranges(ranges);
        // Humidity to Location
        ranges = context.humidity_to_location_map.next_ranges(ranges);
        // location_numbers.push(seed);
        result = result.min(ranges.iter().map(|r|r.start).min().unwrap());
    }

    // Result
    println!("Result of part 2 is {}", result);
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    println!("{:?}", input_file);

    run(input_file);
    run2(input_file);
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_path;

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }

    #[test]
    fn test_input_part_2() {
        let input_path = get_test_input_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
