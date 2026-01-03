use std::fs;

fn main() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    //     let input =
    //         fs::read_to_string("./input_4.txt").expect("Should have been able to read the file");

    println!("{}", part_1(&structure_input(input)));
    println!("{}", part_2(&structure_input(&input)));
}

fn structure_input(input: &str) -> String {
    // add border so we don't have to worry about going off the grid
    let safe_input = input
        .lines()
        .map(|l| format!("O{}0", l))
        .collect::<Vec<String>>()
        .join("\n");
    let padding = vec!["O"; input.split_ascii_whitespace().next().unwrap().len() + 2]
        .into_iter()
        .collect::<String>();
    format!("{}\n{}\n{}", padding, safe_input, padding)
}
fn part_1(safe_input: &str) -> i32 {
    // find all instances of "xmas", could be horizontal, vertical, backwards, diagonal
    let mut ans = 0;
    let lines: Vec<&str> = safe_input.lines().collect();
    let line_count = lines.len();
    for (line_pos, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for (char_pos, char) in chars.iter().enumerate() {
            if *char == 'X' {
                // println!("checking line {} char {} at {}", line_pos, char, char_pos);
                ans += check_horizontal(char_pos, line.chars().collect());
                // search vertically up
                if line_pos >= 4 {
                    ans +=
                        check_vertical(char_pos, lines[line_pos - 3..line_pos].to_owned(), "SAM");
                    // search diagonally up
                    ans += check_diagonal(
                        char_pos,
                        line.len(),
                        lines[line_pos - 3..line_pos].to_owned(),
                        "SAM",
                    );
                }
                // search vertically down
                if line_pos <= line_count - 4 {
                    ans += check_vertical(
                        char_pos,
                        lines[line_pos + 1..line_pos + 4].to_owned(),
                        "MAS",
                    );
                    // search diagonally down
                    ans += check_diagonal(
                        char_pos,
                        line.len(),
                        lines[line_pos + 1..line_pos + 4].to_owned(),
                        "MAS",
                    );
                }
            }
        }
    }
    ans
}

fn check_horizontal(pos: usize, current_line: Vec<char>) -> i32 {
    let mut matches = 0;
    if pos > 3 {
        let backward: String = current_line[pos - 3..pos].iter().collect();
        if backward == "SAM" {
            matches += 1;
        }
    }
    if pos < current_line.len() - 3 {
        let forward: String = current_line[pos + 1..pos + 4].iter().collect();
        if forward == "MAS" {
            matches += 1;
        }
    }
    matches
}

fn check_vertical(char_pos: usize, lines_to_check: Vec<&str>, check_for: &str) -> i32 {
    let mut matches = 0;
    let test_vec: Vec<char> = lines_to_check
        .iter()
        .map(|l| l.chars().nth(char_pos).unwrap())
        .collect();

    let test_string = test_vec.iter().collect::<String>();
    if test_string == check_for {
        matches += 1;
    }
    matches
}

fn check_diagonal(
    char_pos: usize,
    line_len: usize,
    lines_to_check: Vec<&str>,
    check_for: &str,
) -> i32 {
    let mut matches = 0;

    if char_pos <= line_len - 4 {
        let mut test_vec: Vec<char> = vec![];
        // go to the right
        for (i, line) in lines_to_check.iter().enumerate() {
            if check_for == "SAM" {
                test_vec.push(line.chars().nth(char_pos + 3 - i).unwrap());
            }
            if check_for == "MAS" {
                test_vec.push(line.chars().nth(char_pos + i + 1).unwrap());
            }
        }

        let test_string = test_vec.iter().collect::<String>();

        if test_string == check_for {
            matches += 1;
        }
    }
    if char_pos >= 4 {
        let mut test_vec: Vec<char> = vec![];
        // go to the left
        for (i, line) in lines_to_check.iter().enumerate() {
            if check_for == "SAM" {
                test_vec.push(line.chars().nth(char_pos - 3 + i).unwrap());
            }
            if check_for == "MAS" {
                test_vec.push(line.chars().nth(char_pos - i - 1).unwrap());
            }
        }
        let test_string = test_vec.iter().collect::<String>();
        if test_string == check_for {
            matches += 1;
        }
    }
    matches
}

fn part_2(safe_input: &str) -> i32 {
    let mut ans = 0;
    let lines: Vec<&str> = safe_input.lines().collect();

    for (line_pos, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for (char_pos, char) in chars.iter().enumerate() {
            if *char == 'A' {
                // should not go off the grid because we added the border
                if char_pos > 1 && char_pos < line.len() - 1 {
                    ans += find_x_mas(char_pos, lines[line_pos - 1..line_pos + 2].to_owned());
                }
            }
        }
    }
    ans
}

fn find_x_mas(char_pos: usize, lines_to_check: Vec<&str>) -> i32 {
    let mut mas_found = 0;
    let mut sam_found = 0;
    let top_chars: Vec<char> = lines_to_check[0].chars().collect();
    let bottom_chars: Vec<char> = lines_to_check[2].chars().collect();
    if top_chars[char_pos - 1] == 'M' {
        if bottom_chars[char_pos + 1] == 'S' {
            mas_found += 1;
        }
    }
    if top_chars[char_pos + 1] == 'M' {
        if bottom_chars[char_pos - 1] == 'S' {
            mas_found += 1;
        }
    }
    if top_chars[char_pos - 1] == 'S' {
        if bottom_chars[char_pos + 1] == 'M' {
            sam_found += 1;
        }
    }
    if top_chars[char_pos + 1] == 'S' {
        if bottom_chars[char_pos - 1] == 'M' {
            sam_found += 1;
        }
    }

    if mas_found + sam_found > 1 {
        return 1;
    }
    0
}
