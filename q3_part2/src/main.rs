use std::env;
use std::fs;

fn is_num(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn pre_cal_val(i: u32, mut j: u32, m: u32, contents: &String) -> i32 {
    while j>=1 {
        let val_str: char = contents.lines().nth((i) as usize).unwrap_or(&"").chars().nth((j-1) as usize).unwrap_or('\0');
        if is_num(val_str) {
            j-=1;
        } else {
            break;
        }
    }
    // println!("{} {}", i, j);
    let mut num_str: String = "".to_string();
    while j<m {
        let val_str: char = contents.lines().nth((i) as usize).unwrap_or(&"").chars().nth((j) as usize).unwrap_or('\0');
        if is_num(val_str) {
            num_str+=&val_str.to_string();
            j+=1;
        } else {
            break;
        }    
    }
    return num_str.parse::<i32>().unwrap_or(0);
}

fn cal_val(left: char, mid: char, right: char, i: u32, j: u32, m: u32, contents: &String) -> (i32, i32) {
    if left == '.' && mid == '.' && right == '.' {
        return (0,0)
    } else if is_num(left) && mid == '.' && right == '.' {
        // println!("left");
        return (1, pre_cal_val(i,j-1,m,contents));
    } else if left == '.' && is_num(mid) && right == '.' {
        // println!("mid");
        return (1, mid.to_digit(10).unwrap_or(0) as i32);
    } else if left == '.' && mid == '.' && is_num(right) {
        // println!("right");
        return (1, pre_cal_val(i,j+1,m,contents));
    } else if is_num(left) && is_num(mid) && right == '.' {
        // println!("left mid");
        return (1, pre_cal_val(i,j,m,contents));
    } else if is_num(left) && mid == '.' && is_num(right) {
        // println!("left right");
        return (2, pre_cal_val(i,j-1,m,contents)*pre_cal_val(i,j+1,m,contents));
    } else if left == '.' && is_num(mid) && is_num(right) {
        // println!("mid right");
        return (1, pre_cal_val(i,j,m,contents));
    } else {
        // println!("left mid right");
        return (1, pre_cal_val(i,j-1,m,contents));
    }
}

fn row_cal_val(left: char, right: char, i: u32, j: u32, m: u32, contents: &String) -> (i32, i32) {
    if left == '.' && right == '.' {
        return (0,0);
    } else if is_num(left) && right == '.' {
        // println!("left");
        return (1, pre_cal_val(i,j-1,m,contents));
    } else if left == '.' && is_num(right) {
        // println!("right");
        return (1, pre_cal_val(i,j+1,m,contents));
    } else {
        // println!("left right");
        return (2, pre_cal_val(i,j-1,m,contents)*pre_cal_val(i,j+1,m,contents));
    }
}

fn check(i: u32, j: u32, n: u32, m: u32, contents: &String) -> i32 {
    let mut top_cnt = 0;
    let mut top_val = 0;
    if i>=1 {
        let top_left: char = contents.lines().nth((i-1) as usize).unwrap_or(&"").chars().nth((j-1) as usize).unwrap_or('\0'); 
        let top: char = contents.lines().nth((i-1) as usize).unwrap_or(&"").chars().nth((j) as usize).unwrap_or('\0'); 
        let top_right: char = contents.lines().nth((i-1) as usize).unwrap_or(&"").chars().nth((j+1) as usize).unwrap_or('\0');
        (top_cnt, top_val) = cal_val(top_left, top, top_right, i-1, j, m, contents);
    }
    let mut bottom_cnt = 0;
    let mut bottom_val = 0;
    if i<n-1 {
        let bottom_left: char = contents.lines().nth((i+1) as usize).unwrap_or(&"").chars().nth((j-1) as usize).unwrap_or('\0'); 
        let bottom: char = contents.lines().nth((i+1) as usize).unwrap_or(&"").chars().nth((j) as usize).unwrap_or('\0'); 
        let bottom_right: char = contents.lines().nth((i+1) as usize).unwrap_or(&"").chars().nth((j+1) as usize).unwrap_or('\0');
        (bottom_cnt, bottom_val) = cal_val(bottom_left, bottom, bottom_right, i+1, j, m, contents);
    }
    let left: char = contents.lines().nth((i) as usize).unwrap_or(&"").chars().nth((j-1) as usize).unwrap_or('\0');
    let right: char = contents.lines().nth((i) as usize).unwrap_or(&"").chars().nth((j+1) as usize).unwrap_or('\0');
    let (val_cnt, val) = row_cal_val(left, right, i, j, m, contents);
    // println!("{} {} {} {} {} {}", top_cnt, top_val, bottom_cnt, bottom_val, val_cnt, val);
    if top_cnt + bottom_cnt + val_cnt == 2 {
        match (top_cnt, bottom_cnt, val_cnt) {
            (2, 0, 0) => return top_val,
            (0, 2, 0) => return bottom_val,
            (0, 0, 2) => return val,
            (1, 1, 0) => return top_val * bottom_val,
            (1, 0, 1) => return top_val * val,
            (0, 1, 1) => return bottom_val * val,
            _ => return 0,
        }
    } else {
        return 0;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut sum: i32 = 0;
    let n: u32 = contents.lines().count() as u32;
    let m: u32 = contents.lines().nth(0).unwrap_or(&"").len() as u32;
    for i in 0..n {
        for j in 0..m {
            let val_str: char = contents.lines().nth(i as usize).unwrap_or(&"").chars().nth(j as usize).unwrap_or('\0');
            if val_str== '*' {
                let prod: i32 = check(i,j,n,m,&contents) as i32;
                // println!("{} {} {}", i, j, prod);
                sum += prod;
            }
        }
    }
    println!("{}", sum);
}