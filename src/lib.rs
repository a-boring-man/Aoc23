use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_file_bytes(filepath: &str) -> std::io::Result<Vec<u8>> {
    fs::read(filepath)
}

pub fn read_lines_byte<P>(filepath: P) -> std::io::Result<Vec<Vec<u8>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.map(|s| s.into_bytes()))
        .collect()
}

pub fn read_lines_string<P>(filepath: P) -> std::io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}
