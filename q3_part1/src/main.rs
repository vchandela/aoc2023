use std::env;
use std::fs;

fn check(i: u32, j: u32, n: u32, m: u32, contents: &String) -> bool {
    if i>=1 && j>=1 {
        let val_str: char = contents.lines().nth((i-1) as usize).unwrap_or(&"").chars().nth((j-1) as usize).unwrap_or('\0');
        if !(val_str >= '0' && val_str <= '9' || val_str == '.') {
            return true;
        } 
    }
    if i>=1 {
        let val_str: char = contents.lines().nth((i-1) as usize).unwrap_or(&"").chars().nth(j as usize).unwrap_or('\0');
        if !(val_str >= '0' && val_str <= '9' || val_str == '.') {
            return true;
        } 
    }
    if i>=1 && j<m-1 {
        let val_str: char = contents.lines().nth((i-1) as usize).unwrap_or(&"").chars().nth((j+1) as usize).unwrap_or('\0');
        if !(val_str >= '0' && val_str <= '9' || val_str == '.') {
            return true;
        } 
    }
    if j>=1 {
        let val_str: char = contents.lines().nth(i as usize).unwrap_or(&"").chars().nth((j-1) as usize).unwrap_or('\0');
        if !(val_str >= '0' && val_str <= '9' || val_str == '.') {
            return true;
        } 
    }
    if j<m-1 {
        let val_str: char = contents.lines().nth(i as usize).unwrap_or(&"").chars().nth((j+1) as usize).unwrap_or('\0');
        if !(val_str >= '0' && val_str <= '9' || val_str == '.') {
            return true;
        } 
    }
    if i<n-1 && j>=1 {
        let val_str: char = contents.lines().nth((i+1) as usize).unwrap_or(&"").chars().nth((j-1) as usize).unwrap_or('\0');
        if !(val_str >= '0' && val_str <= '9' || val_str == '.') {
            return true;
        } 
    }
    if i<n-1 {
        let val_str: char = contents.lines().nth((i+1) as usize).unwrap_or(&"").chars().nth(j as usize).unwrap_or('\0');
        if !(val_str >= '0' && val_str <= '9' || val_str == '.') {
            return true;
        } 
    }
    if i<n-1 && j<m-1 {
        let val_str: char = contents.lines().nth((i+1) as usize).unwrap_or(&"").chars().nth((j+1) as usize).unwrap_or('\0');
        if !(val_str >= '0' && val_str <= '9' || val_str == '.') {
            return true;
        } 
    }
    return false;

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut sum: i32 = 0;
    let n: u32 = contents.lines().count() as u32;
    let m: u32 = contents.lines().nth(0).unwrap_or(&"").len() as u32;
    // println!("{} {}", n, m);
    for i in 0..n {
        let mut flag: i32 = 0;
        let mut num_str: String = "".to_string();
        for j in 0..m {
            let val_str: char = contents.lines().nth(i as usize).unwrap_or(&"").chars().nth(j as usize).unwrap_or('\0');
            // println!("{} {}", val_str, flag);
            if val_str >= '0' && val_str <= '9' {
                num_str += &val_str.to_string();
                if flag == 0 {
                    flag = 1;
                    // println!("start: {}", num_str);
                } else if flag == 2 {
                    // println!("continue: {}", num_str);
                    continue;
                }
                if check(i,j,n,m,&contents) {
                    flag = 2;
                }
            } else {
                if flag == 2 {
                    let num: i32 = num_str.parse::<i32>().unwrap_or(0);
                    // println!("check: {}", num);
                    sum += num;
                }
                flag = 0;
                num_str = "".to_string();
            }
        }
        if flag == 2 {
            let num: i32 = num_str.parse::<i32>().unwrap_or(0);
            // println!("check: {}", num);
            sum += num;
        }
    }
    println!("{}", sum);
}
