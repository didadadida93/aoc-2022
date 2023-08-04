use std::fs;

fn get_value_from_char(c: char) -> u32 {
    let d = c.to_digit(36).expect("Failed to convert char into digit") - 9;
    if c.is_uppercase() {
        d + 26
    } else {
        d
    }
}

fn contains(w: &str, c: char) -> bool {
    match w.find(c) {
        Some(_) => true,
        None => false,
    }
}

pub fn solve() {
    let total = fs::read_to_string("./input/day-3.txt")
        .expect("Failed to read day-3.txt file")
        .trim()
        .lines()
        .map(|item| {
            let left_side = &item[0..(item.len() / 2)];
            let right_side = &item[(item.len() / 2)..];
            let mut iter = right_side.chars();
            let mut v: Option<char> = None;
            while let Some(c) = iter.next() {
                if contains(left_side, c) {
                    v = Some(c);
                    break;
                }
            }
            let value = match v {
                None => panic!("Unable to find match char"),
                Some(v) => get_value_from_char(v),
            };
            value
        })
        .sum::<u32>();

    let content = fs::read_to_string("./input/day-3.txt").expect("Failed to read day-3.txt file");
    let cloned_content = content.clone();
    let mut iter = cloned_content.trim().lines().enumerate();
    let mut lines: Vec<Vec<&str>> = vec![];
    let mut temp: Vec<&str> = vec![];
    // consume first element
    if let Some(e) = iter.next() {
        temp.push(e.1);
    }
    while let Some(e) = iter.next() {
        if e.0 % 3 == 0 {
            // push to lines
            // empty vec
            // then push it again
            temp.sort_unstable();
            lines.push(temp.clone());
            temp.clear();
            temp.push(e.1);
        } else {
            // push to vec
            temp.push(e.1);
        }
    }
    temp.sort_unstable();
    lines.push(temp.clone());
    let another_total = lines
        .into_iter()
        .map(|item| {
            let first_element = item.get(0).unwrap();
            let second_element = item.get(1).unwrap();
            let third_element = item.get(2).unwrap();
            let mut iter = third_element.chars();
            let mut v: Option<char> = None;
            while let Some(c) = iter.next() {
                if contains(first_element, c) && contains(second_element, c) {
                    v = Some(c);
                    break;
                }
            }
            let value = match v {
                None => panic!("Unable to find match char"),
                Some(v) => get_value_from_char(v),
            };
            value
        })
        .sum::<u32>();
    println!("---------------------------------");
    println!("day 3 puzzle-1 : {}", total);
    println!("day 3 puzzle-2 : {:?}", another_total);
}
