/// this file only open file and return iterator for lines.

use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::path::Path;

fn read_lines(filename:&Path) -> io::Lines<io::BufReader<File>>
{
    let file: Lines<BufReader<File>> = match File::open(filename) {
        Err(err) => panic!("could not open {:?}: {}", filename, err),
        Ok(file) => io::BufReader::new(file).lines(),
    };
    return file;
}

pub fn get_lines_iterator(filename: &str) -> Lines<BufReader<File>> {
    let path = Path::new(filename);
    read_lines(&path)
}