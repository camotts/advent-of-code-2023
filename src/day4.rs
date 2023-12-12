use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::BTreeMap;
use std::collections::HashSet;

type Output = BTreeMap<u32, (u32, HashSet<u32>, HashSet<u32>)>;

fn parse_line(input: &str) -> IResult<&str, (u32, (u32, HashSet<u32>, HashSet<u32>))> {
    let (input, id) = preceded(permutation((tag("Card"), multispace1)), complete::u32)(input)?;
    let (input, (winning, pulled)) = tuple((
        preceded(
            permutation((tag(":"), multispace1)),
            separated_list1(multispace1, complete::u32),
        ),
        preceded(
            permutation((multispace1, tag("|"), multispace1)),
            separated_list1(multispace1, complete::u32),
        ),
    ))(input)?;
    Ok((
        input,
        (
            id,
            (
                1,
                winning.into_iter().collect(),
                pulled.into_iter().collect(),
            ),
        ),
    ))
}

fn parse_input(input: &str) -> IResult<&str, Output> {
    let (input, list) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, list.into_iter().collect()))
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Output {
    let (input, output) = parse_input(input).expect("could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day4, part1)]
pub fn part1(input: &Output) -> u32 {
    let mut ret = 0;
    let base: u32 = 2;
    for (_, v) in input.iter() {
        let l: u32 =
            v.2.intersection(&v.1)
                .collect::<Vec<&u32>>()
                .len()
                .try_into()
                .unwrap();
        ret = ret + base.pow(l - 1);
    }
    ret
}

#[aoc(day4, part2)]
pub fn part2(input: &Output) -> u32 {
    let mut mut_in = input.clone();
    let mut ret = 0;
    let keys: Vec<u32> = mut_in.keys().cloned().collect();
    for k in keys {
        let v = mut_in.get(&k).unwrap();
        let ct = v.0.clone();
        let l: u32 =
            v.2.intersection(&v.1)
                .collect::<Vec<&u32>>()
                .len()
                .try_into()
                .unwrap();
        for i in k..k + l {
            mut_in.get_mut(&(i + 1)).unwrap().0 += ct;
        }
    }
    for (_, v) in mut_in.iter() {
        ret = ret + v.0;
    }
    ret
}
