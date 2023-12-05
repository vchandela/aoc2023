use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut total_sum: i32 = 0;
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let card: &str = parts.get(1).unwrap_or(&"");
        let card_split: Vec<&str> = card.split('|').collect();
        let left_part: Vec<&str> = card_split.get(0).unwrap_or(&"").split(' ').collect();
        // println!("{:?}", left_part);
        let mut my_map: HashMap<&str, i32> = HashMap::new();
        for idx in 0..left_part.len() {
            let sub_str: &str = left_part.get(idx).unwrap_or(&"");
            if sub_str != "" {
                my_map.insert(sub_str, 1);
            }
        }
        // println!("{:?}", my_map);
        let right_part: Vec<&str> = card_split.get(1).unwrap_or(&"").split(' ').collect();
        // println!("{:?}", right_part);
        let mut sum: i32 = 0;
        for idx in 0..right_part.len() {
            let sub_str: &str = right_part.get(idx).unwrap_or(&"");
            if sub_str != "" {
                let val: i32 = my_map.get(&sub_str).copied().unwrap_or(0);
                // println!("{} {}", sub_str, val);
                sum += val;
            }
        }
        // println!("sum: {}", sum);
        if sum >= 1 {
            sum-=1;
            let power: i32 = 2_i32.pow(sum as u32);
            // println!("{}", power);
            total_sum += power;
        }
    }
    println!("{}", total_sum);
}
