use std::io::BufRead;

fn main() {
    println!("Hello, world!");

    let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let example_reader = std::io::BufReader::new(example.as_bytes());
    assert_eq!(8, solve(example_reader.lines()));

    let file = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let answer = solve(reader.lines());
    println!("Problems answer is {}", answer);
}

fn solve_line(line: std::io::Result<String>) -> u32 {
    let line: String = line.unwrap();
    assert!(line.starts_with("Game "));
    let line: &str = line.strip_prefix("Game ").unwrap();
    let (line_num, line) = line.split_once(':').unwrap();
    let line_num = u32::from_str_radix(line_num, 10).unwrap();
    assert!(line.starts_with(" "));

    for set in line.split(';') {
        for ball in set.split(',') {
            let ball = ball.trim();
            // println!("Processing ball={}", ball);
            let (ball_count, ball_type) = ball.split_once(' ').unwrap();
            let ball_count = u32::from_str_radix(ball_count, 10).unwrap();
            let max_allowed_count = match ball_type {
                "red" => 12,
                "green" => 13,
                "blue" => 14,
                _ => panic!("Shouldnt reach here")
            };
            if ball_count > max_allowed_count {
                return 0;
            }
        }
    }

    return line_num;
}

fn solve<T>(lines: std::io::Lines<std::io::BufReader<T>>) -> u32
where
    T: std::io::Read,
{
    lines.map(solve_line).sum()
}
