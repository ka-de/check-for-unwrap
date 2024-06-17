use std::process::exit;
use std::fs::File;
use std::io::{ self, BufRead };
use walkdir::WalkDir;
use regex::Regex;
use crossterm::style::{ Color, Print, ResetColor, SetForegroundColor };
use crossterm::ExecutableCommand;

fn main() -> io::Result<()> {
    let re = Regex::new(r"\.unwrap\(\)").unwrap();
    let mut found_unwrap = false;
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e|
            e
                .path()
                .extension()
                .map_or(false, |ext| ext == "rs")
        ) {
        let path = entry.path();
        let display = path.display();
        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        let reader = io::BufReader::new(file);
        for (num, line) in reader.lines().enumerate() {
            let l = line?;
            if re.is_match(&l) {
                found_unwrap = true;
                let parts: Vec<&str> = l.split(".unwrap()").collect();
                std::io
                    ::stdout()
                    .execute(SetForegroundColor(Color::Magenta))?
                    .execute(Print(format!("{}:{}:", display, num + 1)))?
                    .execute(ResetColor)?
                    .execute(Print(parts[0]))?
                    .execute(SetForegroundColor(Color::Red))?
                    .execute(Print(".unwrap()"))?
                    .execute(ResetColor)?
                    .execute(Print(parts[1]))?
                    .execute(Print("\n"))?;
            }
        }
    }
    if found_unwrap {
        exit(1);
    }
    Ok(())
}
