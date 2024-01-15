use std::fs;

struct SymCoord {
    x: usize,
    y: usize
}

struct NumCoord {
    n: u32,
    x1: usize,
    x2: usize
}

fn main() {
    let mut sum = 0;
    let mut num = String::from("");

    let mut nums: Vec<Vec<NumCoord>> = Vec::new();
    let mut sym_coords: Vec<SymCoord> = Vec::new();
    

    for (y, line) in fs::read_to_string("src/input/day03.txt")
        .unwrap()
        .as_str()
        .lines()
        .collect::<Vec<&str>>()
        .iter().enumerate() {
            let mut nums_in_row: Vec<NumCoord> = Vec::new();

            for (x, ch) in line.chars().collect::<Vec<char>>()
                .iter().enumerate() {
                    
                    if ch.is_digit(10) {
                        num.push(ch.clone());
                    } else {
                        if *ch != '.' {
                            sym_coords.push(SymCoord{ x, y });
                        }

                        if num != "" {
                            let realx = if x == 0 { line.len() } else { x };
                            nums_in_row.push(NumCoord{n: num.parse::<u32>().unwrap(), x1: realx - num.len(), x2: realx - 1});
                            num.clear();
                        }
                    }
                }
        
        nums.push(nums_in_row);
    }

    sym_coords.iter().for_each(|sym| {
        let sx = sym.x;
        let sy = sym.y;

        for i in (sy - 1)..(sy + 2) {
            nums[i].iter().for_each(|num| {
                let nx1 = num.x1;
                let nx2 = num.x2;
                if nx1 == sx || nx1 == sx - 1 || nx1 == sx + 1 || nx2 == sx || nx2 == sx - 1 || nx2 == sx + 1 {
                    sum += num.n;
                }
            })
        }
    });

    println!("{}", sum);
}