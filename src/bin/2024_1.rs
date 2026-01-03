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
    
    //     let mut dial_at_zero = 0;
    //     let mut dial_past_zero: i16 = 0;
    // 
    //     for instruction in input.lines() {
    //         let direction = instruction.chars().next().unwrap();
    //         let steps: String = instruction.chars().skip(1).collect();
    //         let n_steps = steps.as_str().parse::<i16>().unwrap() % 100;
    // 
    //         let turns = (n_steps + position).div_floor(100);
    //         println!("{n_steps} clicks {direction} from {position} {turns} turns total");
    //         println!("{dial_past_zero} past zero");
    // 
    //         position = match direction {
    //             'L' => position - n_steps,
    //             'R' => position + n_steps,
    //             _ => 0, // input error
    //         };
    //         
    //         if position == 0 || position == 100 {
    //             dial_at_zero += 1;
    //         } else {
    //             // do some calculations to check for dial_past_zero
    //             // if old position is negative and new position is positive??
    //         } 

    (vec1, vec2)
}

fn calculate_occurrences(right_list: &Vec<i32>) -> HashMap<i32, i32> {
    let mut occurences = HashMap::new();

    for element in right_list {
        *occurences.entry(*element).or_insert(0) += 1;
    }
    occurences
}
