use std::{fs::read_to_string, error::Error};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_to_string("./input.txt")?;
    let mut lines: Vec<&str> = file.lines().collect(); 
    lines = lines.iter().map(|x| x.split_at(10).1).collect();

    let mut sum = 0;

    let mut cards: Vec<usize> = vec![1; lines.len()];

    for (r, line) in lines.iter().enumerate() {
        let mut win_counter = 0;
        let mut counter: HashMap<usize, usize> = HashMap::new();

        let sets: Vec<String> = line.split(" | ").map(|x| x.to_owned()).collect();
        
        let win_nums: Vec<usize> = sets[0].split(' ')
            .filter(|&x| x != "")
            .map(|x| x.parse().unwrap())
            .collect();

        let check_nums: Vec<usize> = sets[1].split(' ')
            .filter(|&x| x != "")
            .map(|x| x.parse().unwrap())
            .collect();

        for n in check_nums {
            counter.insert(n, 0);
        }

        for n in win_nums {
            if counter.contains_key(&n) { win_counter += 1; }
        }

        for i in 1 ..= win_counter {
            cards[r + i] += cards[r];
        }

        sum += dbg!(cards[r]);
    }
    
    println!("{sum}");

    Ok(())
}

