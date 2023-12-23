#![allow(unused)]

use std::fs::read_to_string;

fn main() {
    let file = read_to_string("./input.txt").unwrap();
    let rows: Vec<String> = file.lines().map(String::from).collect();

    let schematic: Vec<Vec<char>> = rows.iter()
        .map(
            |x| x.chars()
                 .collect()
        )
        .collect();

    let mut sum = 0;

    for y in 0..schematic.len() {
        let mut x = 0;

        while x < schematic[y].len() {
            let row = &schematic[y];

            let mut found_number = false;
            while row[x].is_digit(10) {
                

                if x + 1 != row.len() && !row[x + 1].is_digit(10) {
                    for i in 0..2 {
                        
                        if y + i == 0 || y + i - 1 == schematic.len() { continue; }
                        let y = y + i - 1;

                        for j in 0..2 {
                            
                            if x + j == 0 || x + j - 1 == row.len() { continue; }
                            let x = x + j - 1;

                            let c = schematic[y][x];
                            if !c.is_digit(10) && c != '.' {
                                found_number = true;
                            }
                        }
                    }
                    break;
                }
                x += 1;
            }

            if found_number {
                sum += parse_number(&schematic, y, x);
            }

            x += 1
        }
    }
    println!("{}", sum);
}

fn parse_number(schematic: &Vec<Vec<char>>, y: usize, mut x: usize) -> i32 {
    /*
    loop {
        if x + 1 == schematic[y].len() { break; }
        if !schematic[y][x + 1].is_digit(10) { break; }
        x += 1;
    }
    */

    let mut number = 0;
    let mut num_length = 0;

    while schematic[y][x - num_length].is_digit(10) {
        number += schematic[y][x - num_length].to_digit(10).unwrap() as i32 * (10_i32.pow(num_length as u32));
        num_length += 1;
    }

    return dbg!(number) as i32;
}
