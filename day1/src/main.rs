use utils::read_input;

fn main() {
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

    let it = left_list.iter().zip(right_list.iter());

    let mut total_count: i32 = 0;
    let mut part_two_total: i32 = 0;

    for tup in it {
        total_count += (tup.1 - tup.0).abs();
    }

    for num in left_list.iter() {
        let count = right_list.iter().filter(|n| n == &num).count() as i32;
        part_two_total += num * count;
    }

    println!("Total count: {}", total_count);
    println!("Part two total: {}", part_two_total);
}
