use std::env;
use std::fs;
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let (mut sum, mut line_num): (i32, i32) = (0, 0);
    for line in contents.lines() {
        line_num += 1;
        // println!("{}", line);
        let parts: Vec<&str> = line.split(' ').collect();
        // println!("{:?}", parts);
        let part_len: usize = parts.len();
        let (mut maxx_red, mut maxx_green, mut maxx_blue): (i32, i32, i32) = (0, 0, 0);
        for idx in 1..part_len {
            let sub_str: &str = parts.get(idx).unwrap_or(&"");
            let num_str: &str = parts.get(idx-1).unwrap_or(&"");
            let num: i32 = num_str.parse::<i32>().unwrap_or(0);
            match sub_str {
                "red" | "red," | "red;" => maxx_red = cmp::max(maxx_red,num),
                "green" | "green," | "green;" => maxx_green = cmp::max(maxx_green,num),
                "blue" | "blue," | "blue;" => maxx_blue = cmp::max(maxx_blue,num),
                _ => {}, // Do nothing for other cases
            }
        }
        if maxx_red <= 12 && maxx_green <= 13 && maxx_blue <= 14 {
            sum += line_num;
        }
    }
    println!("{}", sum);
}
