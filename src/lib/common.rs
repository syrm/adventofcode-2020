use std::{
    io::BufReader,
    io::BufRead,
    io::Error,
    fs::File,
    path::Path,
};

pub fn read_input(filename: &Path) -> Result<Vec<String>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn read_input_as_integer(filename: &Path) -> impl Iterator<Item = usize> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok).map(|line| (line.parse::<usize>()).unwrap())
}
