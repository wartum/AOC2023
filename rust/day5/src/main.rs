use std::{fs::File, io::Read, sync::Arc, thread};

const INPUT_FILE: &str = "input.txt";

#[derive(Debug)]
struct MappingRange {
    dst_begin: u64,
    src_begin: u64,
    len: u64
}

impl MappingRange {
    fn is_src_in_range(&self, src: u64) -> bool {
        src >= self.src_begin && src <= self.src_begin + self.len
    }


    fn get_dst(&self, src: u64) -> u64 {
        let offset = src - self.src_begin;
        self.dst_begin + offset
    }

    fn from(str: &str) -> MappingRange {
        let as_integers: Vec<u64> = str.split_whitespace()
            .filter(|w| w.chars().all(|c| c.is_ascii_digit()))
            .map(|w| w.parse().unwrap_or(0))
            .collect();
        MappingRange {
            dst_begin: *as_integers.get(0).unwrap(),
            src_begin: *as_integers.get(1).unwrap(),
            len: *as_integers.get(2).unwrap(),
        }
    }
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<MappingRange>
}

impl Mapping {
    fn get_dst(&self, src: u64) -> u64 {
        for r in self.ranges.iter() {
            if r.is_src_in_range(src) {
                return r.get_dst(src);
            }
        }
        return src;
    }
}

#[derive()]
struct SeedGenerator {
    initial_seed: u64,
    current_seed: u64,
    len: u64,
}

impl Iterator for SeedGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item>{
        if self.current_seed == self.initial_seed + self.len {
            None
        } else {
            self.current_seed += 1;
            Some(self.current_seed-1)
        }
    }
}

struct InputAnalyzer {
    input_content: String
}

impl InputAnalyzer {
    fn from(input_content: String) -> InputAnalyzer {
        InputAnalyzer {
            input_content
        }
    }

    fn get_seeds_generators(&self) -> Vec<SeedGenerator> {
        let mut generators = Vec::new();
        for line in self.input_content.split('\n') {
            if line.starts_with("seeds:") {
                let values: Vec<u64> = line.split_whitespace()
                    .filter(|w| w.chars().all(|c| c.is_ascii_digit()))
                    .map(|w| match w.parse::<u64>() {
                        Ok(v) => v,
                        Err(_) => panic!("Cant parse"),
                    })
                    .collect();
                let mut init_seed = None;
                for v in values {
                    if init_seed.is_none() {
                        init_seed = Some(v);
                    } else {
                        generators.push(SeedGenerator {
                            initial_seed: init_seed.unwrap(),
                            current_seed: init_seed.unwrap(),
                            len: v,
                        });
                        init_seed = None;
                    }
                }
            }
        }
        return generators;
    }

    fn get_map(&self, map_name: &str) -> Mapping {
        let mut ranges = Vec::new();
        let mut flag = false;
        for line in self.input_content.split('\n') {
            if line.is_empty() {
                flag = false;
            }
            if flag {
                ranges.push(MappingRange::from(line));
            }
            if line.starts_with(map_name) && line.ends_with("map:"){
                flag = true;
            }

        }
        Mapping { ranges }
    }

    fn get_seed_to_soil_map(&self) -> Mapping {
        self.get_map("seed-to-soil")
    }

    fn get_soil_to_fertilizer_map(&self) -> Mapping {
        self.get_map("soil-to-fertilizer")
    }

    fn get_fertilizer_to_water_map(&self) -> Mapping {
        self.get_map("fertilizer-to-water")
    }

    fn get_water_to_light_map(&self) -> Mapping {
        self.get_map("water-to-light")
    }

    fn get_light_to_temperature_map(&self) -> Mapping {
        self.get_map("light-to-temperature")
    }

    fn get_temperature_to_humidity_map(&self) -> Mapping {
        self.get_map("temperature-to-humidity")
    }

    fn get_humidity_to_location_map(&self) -> Mapping {
        self.get_map("humidity-to-location")
    }
}

struct SuperMapper {
    seed_to_soil_map: Mapping,
    soil_to_fertilizer_map: Mapping,
    fertilizer_to_water_map: Mapping,
    water_to_light_map: Mapping,
    light_to_temperature_map: Mapping,
    temperature_to_humidity_map: Mapping,
    humidity_to_location_map: Mapping,
}

impl SuperMapper {
    fn get_seed_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil_map.get_dst(seed);
        let fertilizer = self.soil_to_fertilizer_map.get_dst(soil);
        let water = self.fertilizer_to_water_map.get_dst(fertilizer);
        let light = self.water_to_light_map.get_dst(water);
        let temperature = self.light_to_temperature_map.get_dst(light);
        let humidity = self.temperature_to_humidity_map.get_dst(temperature);
        let location = self.humidity_to_location_map.get_dst(humidity);
        return location;
    }
}

fn get_input(input_name: &str) -> String {
    let mut file = File::open(input_name).expect("Cannot open input file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Cannot read input file");
    return buf;
}

fn min_loc_for_seed_generator(seed_generator: SeedGenerator, mapper: &SuperMapper) -> u64 {
    let mut loc_min = u64::MAX;
    for seed in seed_generator.into_iter() {
        let loc = mapper.get_seed_location(seed);
        if loc < loc_min {
            loc_min = loc;
        }
    }
    loc_min
}

fn main() {
    let analyzer = InputAnalyzer::from(get_input(INPUT_FILE));
    let mapper = Arc::new(SuperMapper {
        seed_to_soil_map: analyzer.get_seed_to_soil_map(),
        soil_to_fertilizer_map: analyzer.get_soil_to_fertilizer_map(),
        fertilizer_to_water_map: analyzer.get_fertilizer_to_water_map(),
        water_to_light_map: analyzer.get_water_to_light_map(),
        light_to_temperature_map: analyzer.get_light_to_temperature_map(),
        temperature_to_humidity_map: analyzer.get_temperature_to_humidity_map(),
        humidity_to_location_map: analyzer.get_humidity_to_location_map(),
    });

    // multithreaded
    let mut threads = Vec::new();
    for g in analyzer.get_seeds_generators() {
        let shared_mapper_ptr = mapper.clone();
        threads.push(thread::spawn(move || min_loc_for_seed_generator(g, &shared_mapper_ptr)));
    }

    let mut locations = Vec::new();
    for t in threads {
        match t.join() {
            Ok(l) => locations.push(l),
            Err(_) => {},
        }
    }
    let location = locations.iter().min().unwrap();

    // // singlethreaded
    // let location = analyzer.get_seeds_generators().into_iter()
    //     .map(|g| min_loc_for_seed_generator(g, &mapper))
    //     .min().unwrap();

    println!("{}", location);
}
