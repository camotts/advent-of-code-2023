use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    IResult, Parser,
};
use std::collections::HashMap;

use itertools::Itertools;

type Output = HashMap<(i64, i64), Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Space,
    Galaxy,
}

fn parse_line(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(alt((
        tag("#").map(|_| Tile::Galaxy),
        tag(".").map(|_| Tile::Space),
    )))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    let (input, list) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, list))
}

#[aoc_generator(day11, part1)]
pub fn input_generator(input: &str) -> HashMap<(i64, i64), Tile> {
    let (input, output) = parse_input(input).expect("could not parse input");
    assert!(input.is_empty());
    expand(output, 1)
}

#[aoc_generator(day11, part2)]
pub fn input_generator2(input: &str) -> HashMap<(i64, i64), Tile> {
    let (input, output) = parse_input(input).expect("could not parse input");
    assert!(input.is_empty());
    expand(output, 999999)
}

pub fn expand(map: Vec<Vec<Tile>>, expansion: i64) -> HashMap<(i64, i64), Tile> {
    let mut ret = HashMap::new();
    let mut col_offsets: Vec<bool> = vec![];

    for i in 0..map.len() {
        col_offsets.push(map.iter().all(|v| v.get(i).unwrap() == &Tile::Space))
    }
    let mut offset = 0;
    map.iter().enumerate().for_each(|(i, row)| {
        if row.iter().all(|t| t == &Tile::Space) {
            offset += expansion;
        }
        row.iter().enumerate().fold(0, |acc, (j, tile)| {
            let mut r = acc;
            if col_offsets[j] {
                r += expansion
            }

            ret.insert((i as i64 + offset, j as i64 + r), *tile);
            r
        });
    });
    ret
}

#[aoc(day11, part1)]
pub fn part1(input: &HashMap<(i64, i64), Tile>) -> i64 {
    let targets: Vec<((i64, i64), Tile)> = input
        .iter()
        .filter_map(|((i, j), tile)| match tile {
            Tile::Galaxy => Some(((*i, *j), *tile)),
            _ => None,
        })
        .collect();
    targets
        .iter()
        .combinations(2)
        .map(|pairs| {
            let a = pairs[0].0 .0 as i64 - pairs[1].0 .0 as i64;
            let b = pairs[0].0 .1 as i64 - pairs[1].0 .1 as i64;

            a.abs() + b.abs()
        })
        .sum()
}

#[aoc(day11, part2)]
pub fn part2(input: &Output) -> i64 {
    let targets: Vec<((i64, i64), Tile)> = input
        .iter()
        .filter_map(|((i, j), tile)| match tile {
            Tile::Galaxy => Some(((*i, *j), *tile)),
            _ => None,
        })
        .collect();
    targets
        .iter()
        .combinations(2)
        .map(|pairs| {
            let a = pairs[0].0 .0 as i64 - pairs[1].0 .0 as i64;
            let b = pairs[0].0 .1 as i64 - pairs[1].0 .1 as i64;

            a.abs() as i64 + b.abs() as i64
        })
        .sum()
}
