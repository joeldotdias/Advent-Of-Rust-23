use std::fs;

struct Card {
    win_nums: Vec<u32>,
    my_nums: Vec<u32>
}

fn main() {
    let mut score = 0;

    let mut cards: Vec<Card> = Vec::new();

    fs::read_to_string("src/input/day04.txt").unwrap().as_str().lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect();

            let win_nums = parts[0].split(" ").collect::<Vec<&str>>()
                .iter().filter_map(|w|
                    (w.parse::<u32>().is_ok()).then(|| w.parse::<u32>().unwrap())
                ).collect();
            
            let my_nums = parts[1].split(" ").collect::<Vec<&str>>()
                .iter().filter_map(|w|
                    (w.parse::<u32>().is_ok()).then(|| w.parse::<u32>().unwrap())
                ).collect();

            cards.push(Card{ win_nums, my_nums });
        });
    
    cards.iter().for_each(|card| {
        let mut matches = 0;
        
        card.win_nums.iter().for_each(|w| {
            if card.my_nums.contains(w) {
                matches += 1;
            }
        });

        if matches != 0 {
            score += u32::pow(2, matches - 1);
        }
    });

    println!("{}", score);
}