use std::{io::BufRead, collections::HashMap};

fn main() {
    println!("Hello, world!");

    let example = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let example_reader = std::io::BufReader::new(example.as_bytes());
    assert_eq!(35, solve(example_reader.lines()));

    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let answer = solve(reader.lines());
    println!("Problems answer is {}", answer);
}

#[derive(Debug)]
struct Mapping {
    dst_name: String,
    ranges: Vec<(u64, u64, u64)>,
}

#[derive(Debug)]
struct Problem {
    seeds: Vec<u64>,
    maps: HashMap<String, Mapping>,
}

fn parse_problem<T>(lines: std::io::Lines<std::io::BufReader<T>>) -> Problem
where
    T: std::io::Read,
{
    let mut current_from = String::new();
    let mut current_to = String::new();
    let mut problem: Problem = Problem {seeds: Vec::new(), maps: HashMap::new()};
    for line in lines {
        let line = line.unwrap();
        if line.starts_with("seeds:") {
            assert!(current_to.is_empty());
            assert!(current_from.is_empty());
            let (_, nums) = line.split_once(':').unwrap();
            let nums: Vec<u64> = nums.split_whitespace().map(|x| { u64::from_str_radix(x, 10).unwrap()}).collect();
            problem.seeds = nums;
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let first_char: char = line.chars().next().unwrap();
        if first_char.is_digit(10) {
            assert!(!current_from.is_empty());
            assert!(!current_to.is_empty());
            let nums: Vec<u64> = line.split_whitespace().map(|x| { u64::from_str_radix(x.trim(), 10).unwrap()}).collect();
            assert_eq!(nums.len(), 3);
            problem.maps
                .get_mut(&current_from)
                .unwrap().ranges.push((nums[0], nums[1], nums[2]));
        } else {
            assert!(first_char.is_ascii_lowercase());
            let (map_name, tmp) = line.split_once(' ').unwrap();
            assert_eq!(tmp, "map:");
            let (from, to) = map_name.trim().split_once("-to-").unwrap();
            current_from = from.to_string();
            current_to = to.to_string();
            assert!(!problem.maps.contains_key(&current_from));
            problem.maps.insert(
                current_from.clone(),
                Mapping {
                    dst_name: current_to.clone(),
                    ranges: Vec::new()
                }
            );
        }
    }
    assert!(problem.maps.contains_key("seed"));
    return problem;
}

fn solve_seed(seed: u64, maps: &HashMap<String, Mapping>) -> u64 {
    let mut current_category = "seed";
    let mut current_val = seed;
    while maps.contains_key(current_category) {
        let mapping : &Mapping = maps.get(current_category).unwrap();
        current_category = &mapping.dst_name;
        for (dst_start, src_start, range_len) in &mapping.ranges {
            if &current_val >= src_start && current_val <= src_start + range_len {
                current_val = dst_start + (current_val-src_start);
                break;
            }
        }
    }
    return current_val;
}

fn solve_problem(problem: Problem) -> u64 {
    // println!("Solving problem= {:#?}", problem);
    let answer = problem.seeds.iter()
        .map(|x:&u64| solve_seed(x.clone(), &problem.maps))
        .min().unwrap();
    return answer;
}

fn solve<T>(lines: std::io::Lines<std::io::BufReader<T>>) -> u64
where
    T: std::io::Read,
{
    let problem = parse_problem(lines);
    return solve_problem(problem);
}
