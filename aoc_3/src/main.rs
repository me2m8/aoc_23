#![allow(unused)]

use std::fs::read_to_string;

fn main() {
    let file = read_to_string("./input.txt").unwrap();
    let rows: Vec<String> = file.lines()
                                .map(String::from)
                                .collect();

    let schematic: Vec<Vec<char>> = rows.iter()
        .map( |x| x.chars()
        .collect() )
        .collect();

    let mut sum = 0;

    for y in 0..schematic.len() {
        let row = &schematic[y];
        
        for x in 0..row.len() {
            let c = row[x];

            if c == '*' {
                let mut adjacent_nums: Vec<(usize, usize)> = Vec::new();

                for i in 0..=2 {

                    if y + i == 0 || y + i > schematic.len() { continue; }
                    let y = y + i - 1;

                    let row = &schematic[y];

                    let mut x = x - 1;
                    let mut walker = 0;

                    while walker < 3 {
                        if row[x + walker].is_digit(10) {
                            loop {
                                if x + walker == row.len() - 1 || !row[x + walker + 1].is_digit(10) {
                                    println!("Walking...");
                                    adjacent_nums.push((y, x + walker));
                                    break;
                                }

                                walker += 1;
                            }
                        }
                        walker += 1;
                    }
                }
                if adjacent_nums.len() == 2 {
                   sum += parse_number(&schematic, adjacent_nums); 
                }
            }
        }
    }
    println!("{}", sum);
}

fn parse_number(schematic: &Vec<Vec<char>>, coords: Vec<(usize, usize)>) -> u32 {
    let mut final_number = 1;

    for pair in coords {
        let y = pair.0;
        let x = pair.1;

        let mut number = 0; 
        let mut num_length = 0; 

        while schematic[y][x - num_length].is_digit(10) {
            number += schematic[y][x - num_length].to_digit(10).unwrap() * 10_u32.pow(num_length as u32);
            match x.checked_sub(num_length + 1) {
                None => break,
                Some(x) => num_length += 1,
            }
        }

        final_number *= number;
    }

    return dbg!(final_number);
}
