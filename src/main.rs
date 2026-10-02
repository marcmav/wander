use std::io::{self, Read};
use std::fs::{OpenOptions};
use wander::*;

fn main() -> io::Result<()> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("to_visit.json")?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let mut to_visit = serde_json::from_str::<Websites>(&content)?;
    Ok(())
}
