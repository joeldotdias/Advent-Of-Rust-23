use std::{fs, collections::HashMap};

fn main() {
    let mut sum = 0;

    fs::read_to_string("src/input/day02.txt")
        .unwrap()
        .as_str()
        .lines()
        .for_each(|line| {
            let game_id = line.split(": ").collect::<Vec<&str>>()[0][5..].to_string()
                .parse::<u16>().unwrap();
            let picks = line.split(":").collect::<Vec<&str>>()[1].split(";").collect::<Vec<&str>>();
            
            if is_pick_based(picks.as_slice()) {
                sum = sum + game_id;
            }
        });

    println!("{:?}", sum);
}

fn is_pick_based(picks: &[&str]) -> bool {
    let mut flag = false;
    
    let stupid_lookup = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    picks.to_owned().iter().map(|pick| pick.trim()).collect::<Vec<&str>>()
        .iter().for_each(|colours| {
            colours.split(", ").collect::<Vec<&str>>()
                .iter().for_each(|colour| {
                    let parts = colour.split(" ").collect::<Vec<&str>>();
                    let n = parts[0].parse::<u16>().unwrap();
                    let colour = String::from(parts[1]);
                    
                    if n > *stupid_lookup.get(colour.as_str()).unwrap() {
                        flag = true;
                        return;
                    }
                });
        });
    
    return !flag; 
}