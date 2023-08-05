use std::fs;

// "28-29" -> [28, 29]
fn get_numbers(s: String) -> Vec<u32> {
    let mut v = s.split("-");
    let start = v.next().expect("Failed to get start number");
    let start = start.parse::<u32>().expect("Failed to parse to number");
    let end = v.next().expect("Failed to get end number");
    let end = end.parse::<u32>().expect("Failed to parse to number");
    let mut r: Vec<u32> = vec![];
    for i in start..(end + 1) {
        r.push(i);
    }
    r
}

fn fully_contains(left: &Vec<u32>, right: &Vec<u32>) -> bool {
    left.iter().all(|item| right.contains(item))
}

fn contains_some(left: &Vec<u32>, right: &Vec<u32>) -> bool {
    left.iter().any(|item| right.contains(item))
}

pub fn solve() {
    let content = fs::read_to_string("./input/day-4.txt")
        .expect("Failed to read day-4.txt file")
        .trim()
        .to_string();
    let total = content.lines()
        .map(|item| {
            let mut iter = item.split(",");
            let left = iter.next().expect("Failed to get left side");
            let left = get_numbers(left.to_string());
            let right = iter.next().expect("Failed to get right side");
            let right = get_numbers(right.to_string());
            if fully_contains(&left, &right) || fully_contains(&right, &left) {
                1
            } else {
                0
            }
        })
        .sum::<u32>();
    let another_total = content.lines()
        .map(|item| {
            let mut iter = item.split(",");
            let left = iter.next().expect("Failed to get left side");
            let left = get_numbers(left.to_string());
            let right = iter.next().expect("Failed to get right side");
            let right = get_numbers(right.to_string());
            if contains_some(&left, &right) || contains_some(&right, &left) {
                1
            } else {
                0
            }
        })
        .sum::<u32>();
    println!("---------------------------------");
    println!("day 4 puzzle-1 : {}", total);
    println!("day 4 puzzle-2 : {}", another_total);
}
