use std::fs;

struct Card {
    win_nums: Vec<u32>,
    my_nums: Vec<u32>
}

fn main() {
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

    let mut card_copies =vec![1; cards.len()]; 

    for (index, card) in cards.iter().enumerate() {
        let mut matches = 0;
        let curr_copies = card_copies[index];

        card.win_nums.iter().for_each(|w| {
            if card.my_nums.contains(w) {
                matches += 1;
            }
        });

        for i in 1..(matches + 1) {
            card_copies[index + i] += curr_copies;
        }
    }

    let pile_of_cards: u32 = card_copies.iter().sum();

    println!("{}", pile_of_cards);
}