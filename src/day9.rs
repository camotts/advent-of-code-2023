use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

type Output = Vec<Vec<i32>>;

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, complete::i32)(input)
}

fn parse_input(input: &str) -> IResult<&str, Output> {
    let (input, list) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, list))
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Output {
    let (input, output) = parse_input(input).expect("could not parse input");
    assert!(input.is_empty());
    output
}

#[aoc(day9, part1)]
pub fn part1(input: &Output) -> i32 {
    input
        .iter()
        .map(|hist| {
            let mut extrapolations: Vec<Vec<i32>> = vec![hist.clone()];
            while !extrapolations.last().unwrap().iter().all(|v| *v == 0) {
                extrapolations.push(
                    extrapolations
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect::<Vec<i32>>(),
                );
            }
            let mut ret = 0;
            for v in extrapolations.iter().rev().skip(1) {
                ret = ret + v[v.len() - 1];
            }
            ret
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &Output) -> i32 {
    input
        .iter()
        .map(|hist| {
            let mut extrapolations: Vec<Vec<i32>> = vec![hist.clone()];
            while !extrapolations.last().unwrap().iter().all(|v| *v == 0) {
                extrapolations.push(
                    extrapolations
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect::<Vec<i32>>(),
                );
            }
            let mut ret = extrapolations[extrapolations.len() - 2][0];
            for v in extrapolations.iter().rev().skip(2) {
                ret = v[0] - ret;
            }
            ret
        })
        .sum()
}
