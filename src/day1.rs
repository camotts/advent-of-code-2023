type Output = Vec<String>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Output {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Output) -> i32 {
    input
        .iter()
        .map(|s| {
            let v: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();
            format!("{}{}", v.first().unwrap(), v.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &Output) -> i32 {
    input
        .iter()
        .map(|s| {
            let s = s.replace("one", "o1e");
            let s = s.replace("two", "t2o");
            let s = s.replace("three", "t3ree");
            let s = s.replace("four", "f4ur");
            let s = s.replace("five", "f5ve");
            let s = s.replace("six", "s6x");
            let s = s.replace("seven", "s7even");
            let s = s.replace("eight", "e8ght");
            let s = s.replace("nine", "n9ne");
            let v: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();
            format!("{}{}", v.first().unwrap(), v.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}
