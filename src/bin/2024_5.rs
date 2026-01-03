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
    //     let input =
    //         fs::read_to_string("./input_5.txt".to_string()).expect("Should have been able to read the file");

    // println!("part 1 {}", part_1(&input));
    println!("part 2 {}", part_2(&input));
}

fn parse_input(input: &str) -> (HashMap<u8, Vec<u8>>, Vec<&str>) {
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

    (rules, page_numbers)
}

fn part_1(input: &str) -> u16 {
    let mut ans: u16 = 0;
    let (rules, page_numbers) = parse_input(input);

    // Get correctly-ordered pages
    for line in check_rules(&rules, page_numbers, true) {
        let page_numbers: Vec<u8> = line.split(",").map(|s| s.parse::<u8>().unwrap()).collect();
        // add middle value
        ans += page_numbers[page_numbers.len() / 2] as u16;
    }
    ans
}

fn check_rules<'a>(
    rules: &HashMap<u8, Vec<u8>>,
    page_numbers: Vec<&'a str>,
    return_correct_lines: bool,
) -> Vec<&'a str> {
    let mut output_lines: Vec<&'a str> = Vec::new();

    'lines: for line in page_numbers {
        let mut page_numbers: Vec<u8> = line.split(",").map(|s| s.parse::<u8>().unwrap()).collect();
        page_numbers.reverse();
        for (idx, page_number) in page_numbers.iter().enumerate() {
            if let Some(pages_that_go_after) = rules.get(&page_number) {
                let breaks_rules = page_numbers[idx + 1..]
                    .iter()
                    .any(|p| pages_that_go_after.contains(p));
                if breaks_rules {
                    // println!("{page_number} at {idx} in line {line} breaks rules");
                    if !return_correct_lines {
                        output_lines.push(line);
                    }
                    continue 'lines;
                }
            }
        }
        if return_correct_lines {
            output_lines.push(line);
        }
    }
    println!("check rules output {:?}", output_lines);
    output_lines
}

fn part_2(input: &str) -> u16 {
    let mut ans = 0;
    let (mut rules, page_numbers) = parse_input(input);

    // Reorder incorrect pages
    for line in check_rules(&rules, page_numbers, false) {
        let page_numbers: Vec<u8> = line.split(",").map(|s| s.parse::<u8>().unwrap()).collect();
        let sorted_pages = reorder_pages(&page_numbers, &mut rules);
        ans += sorted_pages[page_numbers.len() / 2] as u16;
    }

    ans
}

fn reorder_pages(line: &Vec<u8>, rules: &mut HashMap<u8, Vec<u8>>) -> Vec<u8> {
    let mut reordered_pages: Vec<u8> = vec![];
    let mut pages_to_check: Vec<u8> = line.clone();
    let mut idx_to_check = 0;

    // go through all the pages until finding one only on one side
    // this means that node has nothing pointing to it
    // then remove all the rules with it on the right

    while idx_to_check < pages_to_check.len() {
        let page = pages_to_check[idx_to_check];
        if rules.contains_key(&page) {
            println!("checking {page} in {:?}", pages_to_check);
            // if page is nowhere on the right
            if rules.values().all(|x| !x.contains(&page)) {
                reordered_pages.push(page);
                pages_to_check.remove(idx_to_check);
                rules.remove(&page);
                continue; // Don't increment i since we removed an element
            } else {
                // If no matches, bump the counter, we assume no loops
                if idx_to_check == pages_to_check.len() - 1 {
                    idx_to_check = 0;
                } else {
                    idx_to_check += 1;
                }
            }
        } else if !rules.contains_key(&page) & rules.values().all(|x| !x.contains(&page)) {
            reordered_pages.push(page);
            pages_to_check.remove(idx_to_check);
            continue; // Don't increment i since we removed an element
        } else {
            idx_to_check += 1;
        }
    }

    println!("{:?}", reordered_pages);
    reordered_pages
}
