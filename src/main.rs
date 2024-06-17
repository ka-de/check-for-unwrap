use walkdir::WalkDir;
use std::fs::File;
use std::io::{ self, BufRead };
use regex::Regex;

fn main() -> io::Result<()> {
    let re = Regex::new(r"\.unwrap\(\)").unwrap();

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
                println!("{}:{}:{}", display, num + 1, l);
            }
        }
    }
    Ok(())
}
