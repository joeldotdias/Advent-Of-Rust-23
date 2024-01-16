use std::fs;

struct Line {
    dest: u64,
    src: u64,
    range: u64
}

fn main() {
    let mut seeds = Vec::<u64>::new();
    let mut guides: Vec<Vec<Line>> = Vec::new();

    for (si, section) in fs::read_to_string("src/input/day05.txt")
        .unwrap()
        .as_str()
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter().enumerate() {
            if si == 0 {
                section.split(": ").collect::<Vec<&str>>()[1]
                    .split(" ").collect::<Vec<&str>>()
                        .into_iter().for_each(|n| {
                            seeds.push(n.parse::<u64>().unwrap());
                        });
            }
            else {
                let mut lines = Vec::<Line>::new();  
                section.lines().collect::<Vec<&str>>().iter()
                    .filter(|line| line.as_bytes()[0].is_ascii_digit()).for_each(|f_line| {
                        let nums = f_line.split(" ").collect::<Vec<&str>>().iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                        lines.push(Line{dest: nums[0], src: nums[1], range: nums[2]});
                });
                guides.push(lines);
            }
    }
    let len = seeds.len();
    
    guides.iter().for_each(|guide| {
        let mut exeps = Vec::<usize>::new();
        guide.iter().for_each(|line| {
            for i in 0..len {
                if exeps.contains(&i) {
                    continue;
                }
                
                if seeds[i] >= line.src && seeds[i] <= (line.src + line.range) {
                    let incr_by = seeds[i] - line.src;
                    seeds[i] = line.dest + incr_by;
                    exeps.push(i);
                }
            }
        });
    });

    let min = seeds.iter().min().unwrap();
    println!("{}", min);
}