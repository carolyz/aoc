use std::collections::HashSet;
use std::fs;

fn main() {
    let input = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

    // let input =
    //     fs::read_to_string("./input_2.txt").expect("Should have been able to read the file");
    let safe_count: i32 = input
        .lines()
        .into_iter()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            is_safe_2(&nums)
        })
        .sum();

    println!("{}", safe_count);
}

fn is_safe_1(nums: &[i32]) -> i32 {
    // println!("array is {:?}", nums);
    let ascending = nums[0] < nums[1];
    let mut prev = nums[0];

    for &n in &nums[1..] {
        let diff = (prev - n).abs();

        let is_duplicate = diff < 1;
        let not_sorted = (ascending && prev > n) || (!ascending && prev < n);
        let diff_too_big = diff > 3;

        if is_duplicate || not_sorted || diff_too_big {
            return 0;
        }
        prev = n;
    }
    1
}

fn is_safe_2(nums: &[i32]) -> i32 {
    let potentially_unsafe = is_safe_1(nums);
    // if report is potentially unsafe, check if we can make it safe by removing one element at a time

    if potentially_unsafe == 0 {
        for idx in 0..nums.len() {
            let new_vec: Vec<i32> = nums
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx)
                .map(|(_, v)| *v)
                .collect();
            let now_safe = is_safe_1(&new_vec);
            if now_safe > 0 {
                return now_safe;
            }
        }
        0
    } else {
        // else report is safe
        1
    }
}
