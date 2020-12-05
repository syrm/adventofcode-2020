use std::{
    io::BufReader,
    io::BufRead,
    fs::File,
    path::Path,
};

pub fn read_input(filename: &Path) -> Result<Vec<String>,  std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}
