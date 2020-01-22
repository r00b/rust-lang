use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = &args[1];
    let num: usize = num.trim().parse()
    .expect("Failed to parse arg");
    palindrome(num);
}

fn palindrome (num: usize) {
    let mut counter: usize = 0;
    let mut curr_num: usize = 11;
    while counter < num {
        if is_palindrome(curr_num) {
            println!("{}", curr_num);
            counter += 1;
        }
        curr_num += 1;
    }
}

// this is very unoptimized
// but good for practice
fn is_palindrome (num: usize) -> bool {
    let str: String = num.to_string();
    let mut start: usize = 0;
    let mut end: usize = str.chars().count();
    let mut left_char = str.chars().nth(start).unwrap();
    let mut right_char = str.chars().nth(end - 1).unwrap();
    // println!("{}, {}", left_char, right_char);
    while start < end {
        if left_char == right_char {
            start += 1;
            left_char = str.chars().nth(start).unwrap();
            end -= 1;
            right_char = str.chars().nth(end - 1).unwrap();
            // println!("{}, {}", left_char, right_char);
        } else {
            break;
        }
    }
    start >= end
}
