use exec::Command;
use rofi::Rofi;
use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::path::Path;

const WARP_RC: &str = "~/.warprc";

fn main() -> Result<()> {
    let path = shellexpand::tilde(WARP_RC).as_ref().to_owned();
    let file = File::open(path)?;

    let (names, paths): (Vec<String>, Vec<String>) = BufReader::new(file)
        .lines()
        .filter_map(|s| s.ok())
        .map(|s| s.split(":").map(str::to_owned).collect::<Vec<_>>())
        .filter(|e| e.len() == 2)
        .map(|v| (v[0].to_owned(), v[1].to_owned()))
        .filter_map(|(name, path)| exists((name, path)))
        .collect::<Vec<(String, String)>>()
        .into_iter()
        .unzip();

    match Rofi::new(&names).run_index() {
        Ok(index) => terminal(&paths[index]),
        Err(rofi::Error::Interrupted) => println!("Interrupted"),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}

fn exists((name, path): (String, String)) -> Option<(String, String)> {
    let fq_path = shellexpand::tilde(&path).as_ref().to_owned();
    match Path::new(&fq_path).is_dir() {
        true => Some((name, fq_path)),
        false => None,
    }
}

fn terminal(path: &String) {
    let _err = Command::new("tilix")
        .arg(format!("--working-directory={}", path))
        .exec();
}
