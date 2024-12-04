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
