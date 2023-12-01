
fn main() -> () {
    println!("Hello, world!");

    let example = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    println!("Examples answer is {}", solve(example.to_string()));

    let content = std::fs::read_to_string("input.txt");
    // println!("{:#?}", content);
    let answer = solve(content.unwrap());
    println!("Problems answer is {}", answer);
}

fn solve(input_str: String) -> u32 {
    input_str.lines().map(|line: &str| -> u32 {
        let left: u32 = line.chars().skip_while(|c:&char| -> bool {
            !c.is_digit(10)
        }).next().unwrap().to_digit(10).unwrap();

        let right: u32 = line.chars().rev().skip_while(|c:&char| -> bool {
            !c.is_digit(10)
        }).next().unwrap().to_digit(10).unwrap();

        assert!(left <= 9);
        assert!(right <= 9);

        // println!("left = {}, right = {}", left, right);
        return left*10 + right;

    }).sum()
}
