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
    println!("part 1 sane version {}", part_1(&input));

}

fn check(x: usize, y: usize, step: (i32,i32), lines: &Vec<&str>, letters_to_check: &[char]) -> i32 {
    let mut new_x = x as i32;
    let mut new_y = y as i32;

    for letter in letters_to_check.iter() {
        new_x += step.0;
        new_y += step.1;
        // make sure we don't go off the grid
        if new_x >= 0 && new_y >=0 && new_y < lines.len() as i32 && new_x < lines[0].len() as i32 {
            if lines[new_y as usize].chars().nth(new_x as usize).unwrap() != *letter {
                return 0 // short circuit if we don't see a match
            }
        } else {
            return 0 // stop looking if we're off the grid
        }
    }
    1
}

fn part_1(input: &str) -> i32 {
    let mut ans = 0;
    let directions = [(1,0),(-1,0),(0,1),(0,-1),(1,1),(-1,-1),(1,-1),(-1,1)];

    let lines: Vec<&str> = input.lines().collect();
    for (line_pos, line) in lines.iter().enumerate() {
        let chars = line.chars().enumerate();
        for (char_pos, char) in chars {
            if char == 'X' {
                // check in 8 directions
                for direction in directions {
                    ans+= check(char_pos, line_pos, direction, &lines, &['M', 'A', 'S']);
                }
            }
        }
    }
    ans
}
