use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_input(file_path: &str) -> Result<Vec<Vec<i32>>, io::Error> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;

    let reader = io::BufReader::new(file);

    let data = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    Ok(data)
}

#[macro_export]
macro_rules! measure_time {
    ($func_name:ident) => {{
        let start = Instant::now();
        let result = $func_name(); // Call the function
        let duration = start.elapsed();
        println!("{} took {:?}", stringify!($func_name), duration);
        result
    }};
}
