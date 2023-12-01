use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut sum = 0;
    for line in contents.lines() {
        // println!("{}", line);
        let mut tens: i32 = -1;
        let mut ones: i32 = -1;
        let length = line.len();
        for idx in 0..length {
            let char = line.chars().nth(idx).unwrap();
            if char >= '0' && char <= '9' {
                tens = char.to_digit(10).unwrap() as i32;
            } 
            if length>=3 && idx <= length-3 {
                let sub_string = line.get(idx..idx+3).unwrap();
                // println!("{}", sub_string);
                match sub_string {
                    "one" => tens = 1,
                    "two" => tens = 2,
                    "six" => tens = 6,
                    _ => {}, // Do nothing for other cases
                }
            } 
            if length>=4 && idx <= length-4 {
                let sub_string = line.get(idx..idx+4).unwrap();
                // println!("{}", sub_string);
                match sub_string {
                    "four" => tens = 4,
                    "five" => tens = 5,
                    "nine" => tens = 9,
                    _ => {}, // Do nothing for other cases
                }
            }  
            if length>=5 && idx <= length-5 {
                let sub_string = line.get(idx..idx+5).unwrap();
                // println!("{}", sub_string);
                match sub_string {
                    "three" => tens = 3,
                    "seven" => tens = 7,
                    "eight" => tens = 8,
                    _ => {}, // Do nothing for other cases
                }
            }
            if tens != -1 {
                break;
            }
        }
        let line_rev = line.chars().rev().collect::<String>();
        for idx in 0..length {
            let char = line_rev.chars().nth(idx).unwrap();
            if char >= '0' && char <= '9' {
                ones = char.to_digit(10).unwrap() as i32;
            }  
            if length>=3 && idx <= length-3 {
                let sub_string = line_rev.get(idx..idx+3).unwrap();
                // println!("{}", sub_string);
                match sub_string {
                    "eno" => ones = 1,
                    "owt" => ones = 2,
                    "xis" => ones = 6,
                    _ => {}, // Do nothing for other cases
                }
            }  
            if length>=4 && idx <= length-4 {
                let sub_string = line_rev.get(idx..idx+4).unwrap();
                // println!("{}", sub_string);
                match sub_string {
                    "ruof" => ones = 4,
                    "evif" => ones = 5,
                    "enin" => ones = 9,
                    _ => {}, // Do nothing for other cases
                }
            }  
            if length>=5 && idx <= length-5 {
                let sub_string = line_rev.get(idx..idx+5).unwrap();
                // println!("{}", sub_string);
                match sub_string {
                    "eerht" => ones = 3,
                    "neves" => ones = 7,
                    "thgie" => ones = 8,
                    _ => {}, // Do nothing for other cases
                }
            }
            if ones != -1 {
                break;
            }
        }
        // println!("{} {}", tens, ones);
        sum += tens*10 + ones;
    }
    println!("Sum: {}", sum);
}
