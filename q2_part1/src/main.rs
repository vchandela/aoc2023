use std::env;
use std::fs;
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut sum = 0;
    let mut line_num = 0;
    for line in contents.lines() {
        // println!("{}", line);
        line_num += 1;
        let parts: Vec<&str> = line.split(' ').collect();
        // println!("{:?}", parts);
        let part_len = parts.len();
        let mut maxx_red = 0;
        let mut maxx_green = 0;
        let mut maxx_blue = 0;
        for idx in 1..part_len {
            let sub_string = parts.get(idx).unwrap();
            // In case of ParseIntError, set num to 0
            let num = parts.get(idx-1).unwrap().parse::<i32>().unwrap_or(0);
            match *sub_string {
                "red" => maxx_red = cmp::max(maxx_red,num),
                "red," => maxx_red = cmp::max(maxx_red,num),
                "red;" => maxx_red = cmp::max(maxx_red,num),
                "green" => maxx_green = cmp::max(maxx_green,num),
                "green," => maxx_green = cmp::max(maxx_green,num),
                "green;" => maxx_green = cmp::max(maxx_green,num),
                "blue" => maxx_blue = cmp::max(maxx_blue,num),
                "blue," => maxx_blue = cmp::max(maxx_blue,num),
                "blue;" => maxx_blue = cmp::max(maxx_blue,num),
                _ => {}, // Do nothing for other cases
            }
        }
        if maxx_red <= 12 && maxx_green <= 13 && maxx_blue <= 14 {
            sum += line_num;
        }
    }
    println!("{}", sum);
}
