#![feature(array_chunks)]
#![feature(iter_array_chunks)]
use rayon::prelude::*;
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

    // let mut result:usize = usize::MAX;
    let result:usize =
    context
        .seeds
        .par_chunks(2)
        .map(|slice| {
            let seed_start = slice[0];
            let seed_length = slice[1];
            let mut inner_result = usize::MAX;
            for mut seed in seed_start..seed_start + seed_length {
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
                // location_numbers.push(seed);
                inner_result = inner_result.min(seed)
            }
            return inner_result;
        }).min().unwrap();

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
