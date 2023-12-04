use std::io::BufRead;

fn main() {
    println!("Hello, world!");

    let example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let example_reader = std::io::BufReader::new(example.as_bytes());
    assert_eq!(13, solve(example_reader.lines()));

    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let answer = solve(reader.lines());
    println!("Problems answer is {}", answer);
}

fn solve_line(line: std::io::Result<String>) -> u32 {
    let line: String = line.unwrap();
    assert!(line.starts_with("Card "));
    let (_, line) = line.split_once(':').unwrap();
    assert!(line.starts_with(" "));
    let (winning_str, has_str) = line.split_once('|').unwrap();
    let mut winning_nums : Vec<u32> = winning_str
        .split_whitespace()
        .map(|x: &str| -> u32 { u32::from_str_radix(x.trim(), 10).unwrap() }).collect();
    let mut has_nums : Vec<u32> = has_str
        .split_whitespace()
        .map(|x: &str| -> u32 { u32::from_str_radix(x.trim(), 10).unwrap() }).collect();
    winning_nums.sort_unstable();
    has_nums.sort_unstable();

    let mut winning_iter = winning_nums.iter().peekable();
    let mut has_iter = has_nums.iter().peekable();
    let mut matches = 0;
    while let (Some(win_num), Some(has_num)) = (winning_iter.peek(), has_iter.peek()) {
        if win_num == has_num {
            matches += 1;
            winning_iter.next();
            has_iter.next();
        }
        else if win_num < has_num {
            winning_iter.next();
        } else {
            has_iter.next();
        }
    }
    if matches == 0 {
        return 0;
    }
    let res = (2 as i32).checked_pow(matches-1).unwrap().try_into().unwrap();
    // println!("winning_nums = {:#?}, has_nums = {:#?}, res = {}", winning_nums, has_nums, res);
    return res;
}

fn solve<T>(lines: std::io::Lines<std::io::BufReader<T>>) -> u32
where
    T: std::io::Read,
{
    lines.map(solve_line).sum()
}
