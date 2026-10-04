use std::io::{self, Read};
use std::fs::{OpenOptions};
use std::process::Command;
use wander::*;

fn main() -> io::Result<()> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("websites.json")?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let mut websites: WebsiteList;
    if content.is_empty() {
        // i must fetch from an API
        websites = WebsiteList::new(Vec::new());
    } else {
        websites = serde_json::from_str::<WebsiteList>(&content)?;
    }

    let mut cmd = Command::new("firefox");
    cmd.arg(websites.get());
    cmd.status()?;

    Ok(())
}
