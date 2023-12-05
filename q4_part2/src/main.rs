use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut total_sum: u32 = 0;
    let mut cnt: HashMap<u32, u32> = HashMap::new();
    let n: u32 = contents.lines().count() as u32;
    for idx in 0..n {
        let line: &str = contents.lines().nth(idx as usize).unwrap_or(&"");
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
        let mut sum: u32 = 0;
        for idx in 0..right_part.len() {
            let sub_str: &str = right_part.get(idx).unwrap_or(&"");
            if sub_str != "" {
                let val: i32 = my_map.get(&sub_str).copied().unwrap_or(0);
                // println!("{} {}", sub_str, val);
                sum += val as u32;
            }
        }
        let entry = cnt.entry(idx+1).or_insert(0);
        *entry += 1;
        let entry_clone = *entry;
        // println!("sum: {} {}", idx+1, sum);
        for i in idx+2..idx+1+sum+1 {
            let temp = cnt.entry(i).or_insert(0);
            *temp += entry_clone;
            // println!("inc: {} {}", i, *temp);
        }        
    }
    for idx in 1..n+1 {
        let val = cnt.entry(idx).or_insert(0);
        total_sum += *val;
        // println!("{} {}", idx, *val);
    }
    println!("{}", total_sum);
}
