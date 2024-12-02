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
