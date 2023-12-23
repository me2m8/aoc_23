#![allow(unused)]

use std::fs::read_to_string;

fn main() {
    let file = read_to_string("./input.txt").unwrap();
    let lines = file.lines().map(String::from);

    let mut sum = 0;

    for (i, line) in lines.enumerate().into_iter() {
        let (_game, all_sets) = line.split_once(": ").unwrap();
        let sets: Vec<&str> = all_sets.split("; ").collect();

        sum += calculate_power(sets);
    }

    println!("{}", sum);
}

fn calculate_power(sets: Vec<&str>) -> i32 {
    let (mut rp, mut gp, mut bp) = (1, 1, 1);

    for set in sets {
        let dices: Vec<&str> = set.split(", ").collect();
        for dice in dices {
            let (number_string, color) = dice.split_once(" ").unwrap();
            let chars: Vec<char> = number_string.chars().collect();

            let mut number = chars[0].to_digit(10).unwrap() as i32;
            if chars.len() == 2 { number += chars[0].to_digit(10).unwrap() as i32 * 9 + chars[1].to_digit(10).unwrap() as i32; }

            match color.chars().nth(0).unwrap() {
                'r' => { if number > rp { rp = number; }},
                'g' => { if number > gp { gp = number; }},
                'b' => { if number > bp { bp = number; }},
                _ => panic!("wut")
            }
        }
    }

    return dbg!(rp * gp * bp)
}
