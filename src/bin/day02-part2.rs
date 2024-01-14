use std::{fs, collections::HashMap};

fn main() {
    let mut sum = 0;

    fs::read_to_string("src/input/day02.txt").unwrap().as_str().lines()
        .for_each(|game| {
            let mut min_balls = HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0)
            ]);

            game.split(": ").collect::<Vec<&str>>()[1]
                .split("; ").collect::<Vec<&str>>()
                .iter().for_each(|pick| {
                    let draws = pick.split(", ").collect::<Vec<&str>>();
                    
                    draws.iter().for_each(|draw| {
                        let parts = draw.split(" ").collect::<Vec<&str>>();
                        let count = parts[0].parse::<u32>().unwrap();
                        let colour = parts[1];
                        
                        if *min_balls.get(colour).unwrap() < count {
                            min_balls.insert(colour, count);
                        }
                    })
                });
        
        sum += min_balls.get("red").unwrap() * min_balls.get("blue").unwrap() * min_balls.get("green").unwrap();
    });

    println!("{:?}", sum);
}