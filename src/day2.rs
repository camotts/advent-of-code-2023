use std::collections::HashMap;
use std::cmp;

type Output = Vec<Game>;

#[derive(Debug, Clone)]
pub struct Game {
    id: i32,
    pulls: Vec<HashMap<String, i32>>
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Output {
    input.lines().map(|s| {
        let split = s.split(":").collect::<Vec<&str>>();
        Game{
            id: split[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),
            pulls: split[1].split(";").collect::<Vec<&str>>().iter().map(|g| {
                let draws = g.trim().split(",").collect::<Vec<&str>>();
                let tuples: Vec<(String, i32)> = draws.iter().map(|d| {
                    let data = d.trim().split(" ").collect::<Vec<&str>>();
                    (data[1].to_string(), data[0].parse::<i32>().unwrap())
                }).collect();

                let m: HashMap<_, _> = tuples.into_iter().collect();
                m
            }).collect()
        }
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Output) -> i32 {
    let constraints: HashMap<String, i32> = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14)
    ]);
    input.iter().filter(|g| {
        g.pulls.iter().all(|p| {
            for (k,v) in p {
                if constraints[k] < *v {
                    return false
                }
            }
            true
        })
    }).map(|g| {
        g.id
    }).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &Output) -> i32 {
    input.iter().map(|g| {
        let default: HashMap<String, i32> = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0)
        ]);
        g.pulls.iter().fold(default, |acc, p| {
            let ret: HashMap<String, i32> = HashMap::from([
                ("red".to_string(), cmp::max(*p.get("red").or(Some(&0)).unwrap(), acc["red"])),
                ("green".to_string(), cmp::max(*p.get("green").or(Some(&0)).unwrap(), acc["green"])),
                ("blue".to_string(), cmp::max(*p.get("blue").or(Some(&0)).unwrap(), acc["blue"]))
            ]);
            ret
        }).into_values().product::<i32>()
    }).sum()
}