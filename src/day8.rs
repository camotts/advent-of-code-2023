use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};
use std::collections::HashMap;

type Output = Data;

pub struct Data {
    instructions: Vec<Turn>,
    map: HashMap<String, (String, String)>,
}

pub enum Turn {
    Left,
    Right,
}
fn parse_input(input: &str) -> IResult<&str, Data> {
    let (input, ins) = terminated(
        many1(alt((
            tag("L").map(|_| Turn::Left),
            tag("R").map(|_| Turn::Right),
        ))),
        line_ending,
    )(input)?;
    let (input, nodes) = preceded(
        line_ending,
        separated_list1(
            line_ending,
            separated_pair(
                take(3_usize).map(String::from),
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(
                        take(3_usize).map(String::from),
                        tag(", "),
                        take(3_usize).map(String::from),
                    ),
                    tag(")"),
                ),
            ),
        ),
    )(input)?;
    Ok((
        input,
        Data {
            instructions: ins,
            map: nodes.into_iter().collect(),
        },
    ))
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Output {
    let (input, output) = parse_input(input).expect("could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day8, part1)]
pub fn part1(input: &Output) -> u32 {
    let mut pos = "AAA".to_string();
    let mut ret = 0;
    loop {
        for t in input.instructions.iter() {
            if pos == "ZZZ".to_string() {
                break;
            }
            ret += 1;
            let opt = input.map.get(&pos).unwrap();
            match t {
                Turn::Left => pos = opt.0.to_string(),
                Turn::Right => pos = opt.1.to_string(),
            }
        }
        if pos == "ZZZ".to_string() {
            break;
        }
    }
    ret
}

#[aoc(day8, part2)]
pub fn part2(input: &Output) -> u64 {
    //try getting the number of steps for each path then get the lcm
    let mut starts: Vec<String> = input
        .map
        .keys()
        .filter_map(|k| {
            if k.ends_with("A") {
                Some(k.clone())
            } else {
                None
            }
        })
        .collect();
    starts
        .iter_mut()
        .map(|s| {
            let mut ct = 0;
            let mut pos = s.clone();
            let mut movement = input.instructions.iter().cycle();
            while !pos.ends_with("Z") {
                ct += 1;
                let m = movement.next().unwrap();
                let opt = input.map.get(&pos).unwrap();
                match m {
                    Turn::Left => pos = opt.0.to_string(),
                    Turn::Right => pos = opt.1.to_string(),
                }
            }
            ct
        })
        .fold(1, |acc, x| lcm(acc, x))
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
