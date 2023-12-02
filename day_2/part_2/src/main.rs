use std::io::BufRead;

fn main() {
    println!("Hello, world!");

    let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let example_reader = std::io::BufReader::new(example.as_bytes());
    assert_eq!(2286, solve(example_reader.lines()));

    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let answer = solve(reader.lines());
    println!("Problems answer is {}", answer);
}

fn solve_line(line: std::io::Result<String>) -> u32 {
    let line: String = line.unwrap();
    assert!(line.starts_with("Game "));
    let line: &str = line.strip_prefix("Game ").unwrap();
    let (_, line) = line.split_once(':').unwrap();
    assert!(line.starts_with(" "));

    let mut min_red_count: u32 = 0;
    let mut min_blue_count: u32 = 0;
    let mut min_green_count: u32 = 0;

    for set in line.split(';') {
        for ball in set.split(',') {
            let ball = ball.trim();
            // println!("Processing ball={}", ball);
            let (ball_count, ball_type) = ball.split_once(' ').unwrap();
            let ball_count = u32::from_str_radix(ball_count, 10).unwrap();
            match ball_type {
                "red" => min_red_count = u32::max(min_red_count, ball_count),
                "green" => min_green_count = u32::max(min_green_count, ball_count),
                "blue" => min_blue_count = u32::max(min_blue_count, ball_count),
                _ => panic!("Shouldnt reach here")
            };
        }
    }

    return min_red_count * min_blue_count * min_green_count;
}

fn solve<T>(lines: std::io::Lines<std::io::BufReader<T>>) -> u32
where
    T: std::io::Read,
{
    lines.map(solve_line).sum()
}
