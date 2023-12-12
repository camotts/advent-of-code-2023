use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    IResult, Parser,
};
use std::collections::HashMap;
use std::collections::HashSet;

type Output = (Vec<Vec<Tile>>, HashMap<(usize, usize), Tile>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartingAnimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn parse_line(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(alt((
        tag("|").map(|_| Tile::Vertical),
        tag("-").map(|_| Tile::Horizontal),
        tag("L").map(|_| Tile::NorthEast),
        tag("J").map(|_| Tile::NorthWest),
        tag("7").map(|_| Tile::SouthWest),
        tag("F").map(|_| Tile::SouthEast),
        tag(".").map(|_| Tile::Ground),
        tag("S").map(|_| Tile::StartingAnimal),
    )))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    let (input, list) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, list))
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Output {
    let _input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    let (input, output) = parse_input(_input).expect("could not parse input");
    assert!(input.is_empty());
    (output.clone(), remove_extra(output))
}

pub fn remove_extra(grid: Vec<Vec<Tile>>) -> HashMap<(usize, usize), Tile> {
    let mut ret = HashMap::new();
    let mut start = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == Tile::StartingAnimal {
                start = (i, j);
            }
        }
    }
    let checks: Vec<(usize, usize, Dir)> = vec![
        (start.0 - 1, start.1, Dir::Up),
        (start.0, start.1 - 1, Dir::Left),
        (start.0 + 1, start.1, Dir::Down),
        (start.0, start.1 + 1, Dir::Right),
    ];
    let val: HashSet<(usize, usize, Tile)> = checks
        .iter()
        .find_map(|v| {
            let mut paths: Vec<(usize, usize, Dir)> = vec![*v];
            let mut looped = false;
            let mut checked: HashSet<(usize, usize, Tile)> = HashSet::new();
            //println!("Start: {:?}", start);
            while let Some(check) = paths.pop() {
                //print!("{:?}", check);
                let next = grid.get(check.0).and_then(|l| {
                    l.get(check.1).and_then(|t| {
                        //print!(" got {:?}", t);
                        checked.insert((check.0, check.1, *t));
                        match t {
                            Tile::StartingAnimal => {
                                looped = true;
                                None::<(usize, usize, Dir)>
                            }
                            Tile::Ground => None::<(usize, usize, Dir)>,
                            Tile::NorthEast => match check.2 {
                                Dir::Down => Some((check.0, check.1 + 1, Dir::Right)),
                                Dir::Left => Some((check.0 - 1, check.1, Dir::Up)),
                                _ => None::<(usize, usize, Dir)>,
                            },
                            Tile::NorthWest => match check.2 {
                                Dir::Down => Some((check.0, check.1 - 1, Dir::Left)),
                                Dir::Right => Some((check.0 - 1, check.1, Dir::Up)),
                                _ => None::<(usize, usize, Dir)>,
                            },
                            Tile::SouthEast => match check.2 {
                                Dir::Up => Some((check.0, check.1 + 1, Dir::Right)),
                                Dir::Left => Some((check.0 + 1, check.1, Dir::Down)),
                                _ => None::<(usize, usize, Dir)>,
                            },
                            Tile::SouthWest => match check.2 {
                                Dir::Up => Some((check.0, check.1 - 1, Dir::Left)),
                                Dir::Right => Some((check.0 + 1, check.1, Dir::Down)),
                                _ => None::<(usize, usize, Dir)>,
                            },
                            Tile::Vertical => match check.2 {
                                Dir::Up => Some((check.0 - 1, check.1, Dir::Up)),
                                Dir::Down => Some((check.0 + 1, check.1, Dir::Down)),
                                _ => None::<(usize, usize, Dir)>,
                            },
                            Tile::Horizontal => match check.2 {
                                Dir::Right => Some((check.0, check.1 + 1, Dir::Right)),
                                Dir::Left => Some((check.0, check.1 - 1, Dir::Left)),
                                _ => None::<(usize, usize, Dir)>,
                            },
                        }
                    })
                });
                match next {
                    Some(g) => paths.push(g),
                    _ => {}
                }
            }
            if looped {
                Some(checked)
            } else {
                None
            }
        })
        .unwrap();
    grid.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, tile)| {
            if val.contains(&(i, j, *tile)) {
                ret.insert((i, j), *tile);
            }
        });
    });
    ret
}

#[aoc(day10, part1)]
pub fn part1(input: &Output) -> u32 {
    input.1.len() as u32 / 2
}

#[aoc(day10, part2)]
pub fn part2(input: &Output) -> u32 {
    0
}
