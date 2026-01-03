use std::collections::HashMap;
use std::fs;

fn main() {
    let input = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";
    // let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let vecs = parse_input(&input);
    let vec1 = vecs.0;
    let vec2 = vecs.1;
    let mut ans = 0;

    for (pos, e) in vec1.iter().enumerate() {
        // ans = ans + (vec1[pos] - vec2[pos]).abs();
        let occ = calculate_occurrences(&vec2);
        ans = ans + e * occ.get(e).unwrap_or(&0);
    }
    println!("answer is {}", ans);
}

// Return input as two sorted vecs
fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut location_ids = line.split_ascii_whitespace();
        vec1.push(location_ids.next().unwrap().parse().unwrap());
        vec2.push(location_ids.next().unwrap().parse().unwrap());
    }

    vec1.sort();
    vec2.sort();

    (vec1, vec2)
}

fn calculate_occurrences(right_list: &Vec<i32>) -> HashMap<i32, i32> {
    let mut occurences = HashMap::new();

    for element in right_list {
        *occurences.entry(*element).or_insert(0) += 1;
    }
    occurences
}
