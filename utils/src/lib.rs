use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn file_to_string_vector(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn file_to_iter(filename: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line"))
}
