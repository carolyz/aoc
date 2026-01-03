fn main() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    
    let ans = turn_dial(input, 50);
    println!("answer is {}", ans);
}

fn turn_dial(input: &str, mut position: u16) -> u16 {
    let mut dial_at_zero = 0;
    
    for instruction in input.lines() {
        let direction = instruction.chars().next().unwrap();
        let steps: String = instruction.chars().skip(1).collect();
        // println!("turning {direction}");
        let n_steps = steps.as_str().parse::<u16>().unwrap() % 100;
        // println!("{n_steps} clicks");

        println!("pos {position}");
        position = match direction {
            'L' => {
                if position < n_steps {
                    let n_steps_back = n_steps - position;
                    100 - n_steps_back
                } else {
                    position - n_steps
                }
            },
            'R' => {
                if position + n_steps > 100 {
                    position + n_steps - 100
                } else {
                    position + n_steps
                }
            },
            _ => 0, // input error
        };
        if position == 0 || position == 100 { dial_at_zero += 1 } 
    }
    dial_at_zero
}
