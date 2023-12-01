use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let file_path = &args[1];
    // println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    // println!("With text:\n{}", contents);
    let mut sum = 0;
    for line in contents.lines() {
        let mut tens: i32 = -1;
        let mut ones: i32 = -1;
        for char in line.chars() {
            if char >= '0' && char <= '9' {
                tens = char.to_digit(10).unwrap() as i32;
                break;
            }
        }
        for char in line.chars().rev() {
            if char >= '0' && char <= '9' {
                ones = char.to_digit(10).unwrap() as i32;
                break;
            }
        }
        // println!("{} {}", tens, ones);
        sum += tens*10 + ones;
    }
    println!("Sum: {}", sum);
}