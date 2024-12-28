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
    let mut ans = 0;
    
    let lines: Vec<&str> = input.lines().collect();
    for (line_pos, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for (char_pos, char) in chars.iter().enumerate() {
            if *char == 'X' {
                // check in 8 directions
                ans += check(char_pos, line_pos, 1, 0, &lines);
                ans += check(char_pos, line_pos, -1, 0, &lines);
                ans += check(char_pos, line_pos, 0, 1, &lines);
                ans += check(char_pos, line_pos, 0, -1, &lines);
                ans += check(char_pos, line_pos, 1, 1, &lines);
                ans += check(char_pos, line_pos, -1, -1, &lines);
                ans += check(char_pos, line_pos, 1, -1, &lines);
                ans += check(char_pos, line_pos, -1, 1, &lines);
            }
        }
    }
    println!("{}", ans);
}

fn check(mut x: usize, mut y: usize, step_x: i32, step_y: i32, lines: &Vec<&str>) -> i32 {
    let mut new_x = x as i32;
    let mut new_y = y as i32;
    
    for letter in ['M','A','S'] {
        new_x += step_x;
        new_y += step_y;
        // make sure we don't go off the grid
        if new_x >= 0 && new_y >=0 && new_y < lines.len() as i32 && new_x < lines[0].len() as i32 {
            if lines[new_y as usize].chars().nth(new_x as usize).unwrap() != letter {
                return 0 // short circuit if we don't see a match
            }
        } else {
            return 0 // stop looking if we're off the grid
        }
    }
    1
}