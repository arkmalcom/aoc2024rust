use std::collections::HashMap;
use std::time::Instant;
use utils::measure_time;
use utils::read_input;

fn day1() {
    let data = match read_input("input/input.txt") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut left_list: Vec<i32> = data.iter().map(|arr| arr[0]).collect();
    let mut right_list: Vec<i32> = data.iter().map(|arr| arr[1]).collect();

    left_list.sort();
    right_list.sort();

    let left_right_pairs = left_list.iter().zip(right_list.iter());

    let mut total_count: i32 = 0;
    let mut part_two_total: i32 = 0;

    for tup in left_right_pairs {
        total_count += (tup.1 - tup.0).abs();
    }

    let mut right_map: HashMap<i32, i32> = HashMap::new();
    for &num in right_list.iter() {
        *right_map.entry(num).or_insert(0) += 1;
    }

    for num in left_list.iter() {
        if let Some(&count) = right_map.get(&num) {
            part_two_total += num * count;
        }
    }

    println!("Total count: {}", total_count);
    println!("Part two total: {}", part_two_total);
}

fn main() {
    measure_time!(day1);
}
