type Output = Data;

#[derive(Debug, Clone)]
pub struct Data {
    time: Vec<u64>,
    distance: Vec<u64>,
}

#[aoc_generator(day6, part1)]
pub fn input_generator(input: &str) -> Output {
    let d: Vec<Vec<u64>> = input
        .lines()
        .map(|l| {
            let split = l.split(":").collect::<Vec<&str>>();
            split[1]
                .split_whitespace()
                .map(|c| c.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    Data {
        time: d[0].clone(),
        distance: d[1].clone(),
    }
}

#[aoc_generator(day6, part2)]
pub fn input_generator2(input: &str) -> Output {
    let d: Vec<u64> = input
        .lines()
        .map(|l| {
            let split = l.split(":").collect::<Vec<&str>>();
            split[1].replace(" ", "").parse::<u64>().unwrap()
        })
        .collect();
    Data {
        time: vec![d[0].clone()],
        distance: vec![d[1].clone()],
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &Output) -> u64 {
    input
        .time
        .iter()
        .zip(input.distance.iter())
        .map(|(time, dist)| {
            let mut l = time.div_ceil(2);
            let mut r = time / 2;
            while l * r > *dist {
                l -= 1;
                r += 1;
            }
            (r - 1) - (l + 1) + 1
        })
        .product()
}

#[aoc(day6, part2)]
pub fn part2(input: &Output) -> u64 {
    input
        .time
        .iter()
        .zip(input.distance.iter())
        .map(|(time, dist)| {
            let mut l = time.div_ceil(2);
            let mut r = time / 2;
            while l * r > *dist {
                l -= 1;
                r += 1;
            }
            (r - 1) - (l + 1) + 1
        })
        .product()
}
