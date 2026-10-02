use std::io::{self, Read};
use std::fs::{OpenOptions};
use std::process::Command;
use wander::*;

fn main() -> io::Result<()> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("to_visit.json")?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let mut to_visit: WebsiteList;
    if content.is_empty() {
        // i must fetch from an API
        to_visit = WebsiteList::new(Vec::new());
    } else {
        to_visit = serde_json::from_str::<WebsiteList>(&content)?;
    }

    let mut cmd = Command::new("firefox");
    cmd.arg(to_visit.get());
    cmd.status()?;

    Ok(())
}
