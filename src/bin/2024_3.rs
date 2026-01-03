use regex::Regex;
use std::fs;

fn main() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(get_multiples(input), 161);

    // let input_real = fs::read_to_string("./input_3.txt")
    //     .expect("Should have been able to read the file");
    // println!("{}", get_multiples(&input_real));
}
fn get_multiples(input: &str) -> i32 {
    let mut total = 0;

    let re = Regex::new(r"(?:mul\()(\d+,\d+)(?:\))").unwrap(); // part 1
    let results: Vec<_> = re.find_iter(input).map(|m| m.as_str()).collect();
    println!("{:?}", results);

    // Multiply remaining numbers
    for r in results {
        let cleaned: Vec<&str> = r
            .strip_prefix("mul(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(",")
            .collect();
        // println!("{:?}", cleaned);
        total += cleaned[0].parse::<i32>().unwrap() * cleaned[1].parse::<i32>().unwrap()
    }
    total
}
