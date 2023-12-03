type Output = Vec<Vec<Cell>>;

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Digit(u32),
    Symbol,
    Gear
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Output {
    input.lines().map(|s| s.chars().map(|c| {
        if c.is_ascii_digit() {
            Cell::Digit(c.to_digit(10).unwrap())
        } else if c == '.' {
            Cell::Empty
        } else if c == '*' {
            Cell::Gear
        } else {
            Cell::Symbol
        }
    }).collect()).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &Output) -> u32 {
    let mut ret = 0;
    for (i, line) in input.iter().enumerate() {
        let mut val: u32 = 0;
        let mut part = false;
        for (j, c) in line.iter().enumerate() {
            match c {
                Cell::Digit(num) => {
                    val = (val * 10) + num;
                    if !part {
                       if check_any_adjacent(input, i, j) {
                        part = true;
                       }
                    }
                },
                _ => {
                    if part {
                        ret = ret + val;
                    }
                    part = false;
                    val = 0;
                },
            }
        }
        if part {
            ret = ret + val;
        }
    }
    ret
}

pub fn check_any_adjacent(input: &Output, i: usize, j: usize) -> bool {
    if check(input, i-1, j) {
            return true
    }
    if check(input, i-1, j+1) {
            return true
    }
    if check(input, i, j+1) {
            return true
    }
    if check(input, i+1, j+1) {
            return true
    }
    if check(input, i+1, j) {
            return true
    }
    if check(input, i+1, j-1) {
            return true
    }
    if check(input, i, j-1) {
            return true
    }
    if check(input, i-1, j-1) {
            return true
    }
    return false
}

pub fn check(input: &Output, i: usize, j: usize) -> bool {
    match input.get(i) {
        Some(l) => {
            match l.get(j) {
                Some(c) => {
                    matches!(*c, Cell::Symbol | Cell::Gear)
                },
                None => false
            }
        },
        None => false
    }
}

pub fn check2(input: &Output, i: usize, j: usize) -> bool {
    match input.get(i) {
        Some(l) => {
            match l.get(j) {
                Some(c) => {
                    matches!(*c, Cell::Digit(_))
                },
                None => false
            }
        },
        None => false
    }
}

pub fn get_number(input: &Output, i: usize, j: usize) -> u32 {
    let line = input[i].clone();
    let left = (0..=j).rev().take_while(|idx| matches!(line[*idx], Cell::Digit(_))).min().expect("");
    let right = (j..=line.len()-1).take_while(|idx| matches!(line[*idx], Cell::Digit(_))).max().expect("");
    line[left..=right].iter().map(|c| {
       match c {
        Cell::Digit(x) => x,
        _ => unreachable!()
       }
    }).fold(0, |acc, x| {
        return acc * 10 + x
    })
}

#[aoc(day3, part2)]
pub fn part2(input: &Output) -> u32 {
    let inp = input.clone();
    let mut ret = 0;
    for (i, line) in inp.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            match c {
                Cell::Gear => {
                    let mut nums: Vec<u32> = vec!();

                    if nums.len() < 2 && check2(&inp, i-1, j) {
                        nums.push(get_number(input, i-1, j));
                        
                    }
                    if nums.len() < 2 && check2(&inp, i+1, j) {
                        nums.push(get_number(input, i+1, j));
                    }
                    if nums.len() < 2 && check2(&inp, i, j-1) {
                        nums.push(get_number(input, i, j-1));
                    }
                    if nums.len() < 2 && check2(&inp, i, j+1) {
                        nums.push(get_number(input, i, j+1));
                    }

                    if nums.len() < 2 && check2(&inp, i-1, j-1) && !check2(&inp, i-1, j) {
                        nums.push(get_number(input, i-1, j-1));
                    }
                    if nums.len() < 2 && check2(&inp, i-1, j+1) && !check2(&inp, i-1, j) {
                        nums.push(get_number(input, i-1, j+1));
                    }
                    if nums.len() < 2 && check2(&inp, i+1, j-1) && !check2(&inp, i+1, j){
                        nums.push(get_number(input, i+1, j-1));
                    }
                    if nums.len() < 2 && check2(&inp, i+1, j+1) && !check2(&inp, i+1, j) {
                        nums.push(get_number(input, i+1, j+1));
                    }
                    if nums.len() == 2 {
                        ret = ret + nums.iter().product::<u32>()
                    }
                },
                _ => {}
            }
        }
    }
    ret
}