use std::fs;

fn main() {
    let mut sum = 0;

    fs::read_to_string("src/input/day01.txt")
        .unwrap()
        .as_str()
        .lines()
        .for_each(|line| {
            let line = line
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");
            
            sum += get_num_from_line(&line);
        });

    println!("{}", sum);
}

fn get_num_from_line(line: &str) -> u32 {
    let mut left_most = 0;
    let mut right_most = 0;

    line.chars().for_each(|ch| {        
        if ch.is_digit(10) {
            let num = ch.to_digit(10).unwrap();
            
            if left_most == 0 {
                left_most = num;
                right_most = num;
            } else {
                right_most = num;
            }
        };
    });

    left_most * 10 + right_most
}