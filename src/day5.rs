use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};
use rayon::prelude::*;

type Output = Almanac;

#[derive(Debug, Clone)]
pub struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Data>,
    soil_to_fertilizer: Vec<Data>,
    fertilizer_to_water: Vec<Data>,
    water_to_light: Vec<Data>,
    light_to_tempurature: Vec<Data>,
    temperature_to_humidity: Vec<Data>,
    humidity_to_location: Vec<Data>,
}

#[derive(Debug, Clone)]
pub struct Data {
    source_start: u64,
    destination_start: u64,
    range: u64,
}

fn parse_block<'a>(input: &'a str, tag_str: &'a str) -> IResult<&'a str, Vec<Data>> {
    let (input, mut data) = preceded(
        permutation((tag("\n\n"), tag(tag_str))),
        separated_list1(
            line_ending,
            separated_list1(tag(" "), complete::u64).map(|v| Data {
                source_start: v[1],
                destination_start: v[0],
                range: v[2],
            }),
        ),
    )(input)?;
    data.sort_by(|a, b| {
        a.destination_start
            .partial_cmp(&b.destination_start)
            .unwrap()
    });
    Ok((input, data))
}

fn parse_input(input: &str) -> IResult<&str, Output> {
    let (input, seeds) = preceded(tag("seeds: "), separated_list1(tag(" "), complete::u64))(input)?;
    let (input, seed_to_soil) = parse_block(input, "seed-to-soil map:\n")?;
    let (input, soil_to_fertilizer) = parse_block(input, "soil-to-fertilizer map:\n")?;
    let (input, fertilizer_to_water) = parse_block(input, "fertilizer-to-water map:\n")?;
    let (input, water_to_light) = parse_block(input, "water-to-light map:\n")?;
    let (input, light_to_tempurature) = parse_block(input, "light-to-temperature map:\n")?;
    let (input, temperature_to_humidity) = parse_block(input, "temperature-to-humidity map:\n")?;
    let (input, humidity_to_location) = parse_block(input, "humidity-to-location map:\n")?;

    Ok((
        input,
        Almanac {
            seeds,
            seed_to_soil: seed_to_soil,
            soil_to_fertilizer: soil_to_fertilizer,
            fertilizer_to_water: fertilizer_to_water,
            water_to_light: water_to_light,
            light_to_tempurature: light_to_tempurature,
            temperature_to_humidity: temperature_to_humidity,
            humidity_to_location: humidity_to_location,
        },
    ))
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Output {
    let (input, output) = parse_input(input).expect("could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day5, part1)]
pub fn part1(input: &Output) -> Option<u64> {
    input
        .seeds
        .iter()
        .map(|seed| {
            let soil = calculate_destination(*seed, &input.seed_to_soil);
            let fertilizer = calculate_destination(soil, &input.soil_to_fertilizer);
            let water = calculate_destination(fertilizer, &input.fertilizer_to_water);
            let light = calculate_destination(water, &input.water_to_light);
            let temperature = calculate_destination(light, &input.light_to_tempurature);
            let humidity = calculate_destination(temperature, &input.temperature_to_humidity);
            let location = calculate_destination(humidity, &input.humidity_to_location);
            location
        })
        .min()
}

fn calculate_destination(source: u64, map: &Vec<Data>) -> u64 {
    for item in map {
        if item.source_start <= source && source <= item.source_start + item.range - 1 {
            return source + item.destination_start - item.source_start;
        }
    }
    source
}

#[aoc(day5, part2)]
pub fn part2a(input: &Output) -> Option<u64> {
    input
        .seeds
        .par_iter()
        .step_by(2)
        .zip(input.seeds.par_iter().skip(1).step_by(2))
        .map(|(seed, range)| {
            (*seed..=seed + range).into_par_iter().map(|i| {
                let soil = calculate_destination(i, &input.seed_to_soil);
                let fertilizer = calculate_destination(soil, &input.soil_to_fertilizer);
                let water = calculate_destination(fertilizer, &input.fertilizer_to_water);
                let light = calculate_destination(water, &input.water_to_light);
                let temperature = calculate_destination(light, &input.light_to_tempurature);
                let humidity = calculate_destination(temperature, &input.temperature_to_humidity);
                let location = calculate_destination(humidity, &input.humidity_to_location);
                location
            })
        })
        .flatten()
        .min()
}
