
fn main() -> () {
    println!("Hello, world!");

    let example = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(281, solve(example.to_string()));

    let content = std::fs::read_to_string("input.txt");
    // println!("{:#?}", content);
    let answer = solve(content.unwrap());
    println!("Problems answer is {}", answer);
}

const DIGIT_STRINGS : [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn get_two_digit_num(line: &str) -> u32 {
    let n = line.chars().count();
    let mut left: u32 = 0;
    'outer: for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            left = c.to_digit(10).unwrap();
            break 'outer;
        }
        for j in 0..DIGIT_STRINGS.len() {
            let digit_string = DIGIT_STRINGS[j];
            let eq = digit_string == line.get(i..i+digit_string.len()).unwrap_or("");
            if eq {
                left = u32::try_from(j+1).unwrap();
                break 'outer;
            }
        }
    }

    let mut right: u32 = 0;
    'outer: for (i, c) in line.chars().rev().enumerate() {
        if c.is_digit(10) {
            right = c.to_digit(10).unwrap();
            break 'outer;
        }
        for j in 0..DIGIT_STRINGS.len() {
            let digit_string = DIGIT_STRINGS[j];
            let input_substr = line.get(n-1-i..n-1-i+digit_string.len()).unwrap_or("");
            let eq = digit_string == input_substr;
            if eq {
                right = u32::try_from(j+1).unwrap();
                break 'outer;
            }
        }
    }

    assert!(left <= 9);
    assert!(right <= 9);

    // println!("left = {}, right = {}", left, right);
    return left*10 + right;
}

fn solve(input_str: String) -> u32 {
    input_str.lines().map(get_two_digit_num).sum()
}
