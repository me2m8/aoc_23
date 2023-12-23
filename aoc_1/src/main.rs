#![allow(unused)]

use std::fs::read_to_string;

fn main() {
    let lines = read_to_string("./input.txt").unwrap();
    let lines_iterator = lines.lines().map(String::from);

    let mut sum = 0;

    for line in lines_iterator {
        println!("{}", &line);

        let mut first = -1;
        let mut last = 0;

        for (i, c) in line.chars().enumerate().into_iter() {
            if c.is_numeric() {
                if first == -1 { first = i as i32; }
                last = i as i32;
            }                
        }

        dbg!(first);
        dbg!(last);

        let (first_n, last_n) = find_last_number(line, first, last);

        sum += first_n * 10 + last_n - 0x210;
        println!("{}", sum);

        break;
    }

}

fn find_last_number(s: String, mut first_i: i32, mut last_i: i32) -> (i32, i32) {
    let mut last_n: i32 = s.chars().nth(last_i as usize).unwrap() as i32;
    let mut first_n: i32 = s.chars().nth(first_i as usize).unwrap() as i32;

    dbg!(first_n);
    dbg!(last_n);

    let last_num_vec = vec![
        (s.rfind("zero").unwrap_or(usize::MIN) as i32, 30),
        (s.rfind("one").unwrap_or(usize::MIN) as i32, 31),
        (s.rfind("two").unwrap_or(usize::MIN) as i32, 32),
        (s.rfind("three").unwrap_or(usize::MIN) as i32, 33),
        (s.rfind("four").unwrap_or(usize::MIN) as i32, 34),
        (s.rfind("five").unwrap_or(usize::MIN) as i32, 35),
        (s.rfind("six").unwrap_or(usize::MIN) as i32, 36),
        (s.rfind("seven").unwrap_or(usize::MIN) as i32, 37),
        (s.rfind("eight").unwrap_or(usize::MIN) as i32, 38),
        (s.rfind("nine").unwrap_or(usize::MIN) as i32, 39),
    ]; 

    let first_num_vec = vec![
        (s.rfind("zero").unwrap_or(i32::MAX as usize) as i32, 30),
        (s.rfind("one").unwrap_or(i32::MAX as usize) as i32, 31),
        (s.rfind("two").unwrap_or(i32::MAX as usize) as i32, 32),
        (s.rfind("three").unwrap_or(i32::MAX as usize) as i32, 33),
        (s.rfind("four").unwrap_or(i32::MAX as usize) as i32, 34),
        (s.rfind("five").unwrap_or(i32::MAX as usize) as i32, 35),
        (s.rfind("six").unwrap_or(i32::MAX as usize) as i32, 36),
        (s.rfind("seven").unwrap_or(i32::MAX as usize) as i32, 37),
        (s.rfind("eight").unwrap_or(i32::MAX as usize) as i32, 38),
        (s.rfind("nine").unwrap_or(i32::MAX as usize) as i32, 39),
    ]; 

    dbg!(&first_num_vec);
    dbg!(&last_num_vec);

    dbg!(first_i);
    dbg!(last_i);

    let mut first_found = false;
    let mut last_found = false;

    for (i, n) in last_num_vec {
        if i > last_i {
            last_n = n; 
            last_i = i; 
            last_found = true;
            continue; 
        }
    }
    if !last_found { last_n = s.chars().nth(last_i as usize).unwrap() as i32 }

    for (i, n) in first_num_vec {
        if i < first_i { 
            first_n = n; 
            first_i = i; 
            first_found = true;
            continue; 
        }
    }
    if !first_found { first_n = s.chars().nth(first_i as usize).unwrap() as i32}

    dbg!(first_n);
    dbg!(last_n);

    (first_n, last_n)
}
