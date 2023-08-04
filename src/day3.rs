use std::fs;
use std::ops::ControlFlow;

fn get_value_from_char(c: char) -> u32 {
    let d = c.to_digit(36).expect("Failed to convert char into digit") - 9;
    if c.is_uppercase() {
        d + 26
    } else {
        d
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
            let cf = right_side.chars().try_for_each(|c| {
                if left_side.contains(c) {
                    return ControlFlow::Break(get_value_from_char(c));
                }
                ControlFlow::Continue(())
            });
            match cf {
                ControlFlow::Break(v) => v,
                _ => 0,
            }
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
            let cf = third_element.chars().try_for_each(|c| {
                if first_element.contains(c) && second_element.contains(c) {
                    return ControlFlow::Break(get_value_from_char(c));
                }
                ControlFlow::Continue(())
            });
            match cf {
                ControlFlow::Break(v) => v,
                _ => 0,
            }
        })
        .sum::<u32>();
    println!("---------------------------------");
    println!("day 3 puzzle-1 : {}", total);
    println!("day 3 puzzle-2 : {:?}", another_total);
}
