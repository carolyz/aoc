use std::collections::HashMap;
use std::fs;

fn main() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    // let input =
    //     fs::read_to_string("./input_5.txt").expect("Should have been able to read the file");

    println!("part 1 {}", part_1(&input));
}

fn part_1(input: &str) -> u16 {
    let mut ans:u16 = 0;
    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut page_numbers: Vec<&str> = Vec::new();
    
    for line in input.lines() {
        if line.contains("|") {
            let mut rule = line.split("|");
            let k = rule.next().unwrap().parse::<u8>().unwrap();
            let v = rule.next().unwrap().parse::<u8>().unwrap();
            
            rules.entry(k).or_insert(Vec::new()).push(v);
        } else if line.contains(",") {
            page_numbers.push(line);
        }
    }
    
    for line in check_rules(&rules, page_numbers) {
        // add middle value
        let page_numbers: Vec<u8> = line.split(",").map(|s| s.parse::<u8>().unwrap()).collect();
        
        ans += page_numbers[page_numbers.len() / 2] as u16;
    }
    ans
}

fn check_rules<'a>(rules: &HashMap<u8, Vec<u8>>, page_numbers:Vec<&'a str>) -> Vec<&'a str> {
    let mut correct_lines: Vec<&'a str> = Vec::new();
    'lines: for line in page_numbers {
        let mut page_numbers: Vec<u8> = line.split(",").map(|s| s.parse::<u8>().unwrap()).collect();
        page_numbers.reverse();
        for (idx, page_number) in page_numbers.iter().enumerate() {
            if let Some(pages_that_go_after) = rules.get(&page_number) {
                let breaks_rules = page_numbers[idx+1..].iter().any(|p| pages_that_go_after.contains(p));
                if breaks_rules {
                    continue 'lines;
                }
            }
        }
        correct_lines.push(line);
    }
    correct_lines
}
