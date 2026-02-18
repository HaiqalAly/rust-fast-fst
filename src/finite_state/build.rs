use std::fs::File;
use std::io::{self};
use std::io::{BufRead, BufReader};

use fst::MapBuilder;

pub fn build_fst(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let writer = io::BufWriter::new(File::create(output_path)?);
    let mut build = MapBuilder::new(writer)?;

    while reader.read_line(&mut line)? > 0 {
        {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                line.clear();
                continue;
            }

            if let Some((word, weight_str)) = trimmed.split_once(',') {
                let key = word.trim();
                // Parse and insert only if valid
                if let Ok(value) = weight_str.trim().parse::<u64>() {
                    build.insert(key, value)?;
                }
            } else {
                // Handle lines with no comma
                build.insert(trimmed, 0)?;
            }
        }
        line.clear();
    }

    build.finish()?;
    Ok(())
}
