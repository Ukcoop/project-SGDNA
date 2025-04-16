use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use sgdna_lib::objects::gb_container::{Feature, GBContainer};

fn process_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut state = "";
    let mut name = "".to_string();
    let mut structure = "".to_string();
    let mut features: Vec<Feature> = Vec::new();
    let mut sequences: Vec<&str> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 5 {
                name = parts[1].to_string();
                structure = parts[5].to_string();
            }
        }

        if line.contains("FEATURES") {
            state = "FEATURES";
            continue;
        }
        if line.contains("ORIGIN") {
            state = "ORIGIN";
            continue;
        }

        match state {
            "FEATURES" => {
                if line.chars().nth(5) != Some(' ') {
                    let trimmed = line.trim();
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() < 2 {
                        continue;
                    }

                    let feature_name = parts[0];
                    let range = parts[1];

                    let (start, end) = if range.starts_with("complement(") {
                        let inner = &range["complement(".len()..range.len() - 1];
                        let positions: Vec<&str> = inner.split("..").collect();
                        if positions.len() != 2 {
                            eprintln!("Invalid range format for complement: {}", range);
                            continue;
                        }
                        (positions[0], positions[1])
                    } else {
                        let positions: Vec<&str> = range.split("..").collect();
                        if positions.len() != 2 {
                            eprintln!("Invalid range format: {}", range);
                            continue;
                        }
                        (positions[0], positions[1])
                    };

                    if feature_name != "CDS"
                        && feature_name != "RBS"
                        && feature_name != "terminator"
                        && feature_name != "promoter"
                        && feature_name != "protein_bind"
                        && feature_name != "primer_bind"
                        && feature_name != "rep_origin"
                    {
                        continue;
                    }

                    features.push(Feature {
                        name: feature_name.to_string(),
                        start: start.parse().unwrap_or(0),
                        end: end.parse().unwrap_or(0),
                    });
                }
            }
            "ORIGIN" => {
                if line.len() < 10 {
                    continue;
                }
                let sequence_part = &line[10..];
                let parts: Vec<&str> = sequence_part.split_whitespace().collect();
                for part in parts {
                    if !part.is_empty() {
                        sequences.push(part);
                    }
                }
            }
            _ => {}
        }
    }

    let gb = GBContainer {
        name,
        structure,
        features,
        dna: sequences.join(""),
    };

    fs::create_dir_all("test_data")?;

    let output_file_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let output_path = format!("test_data/{}.json", output_file_name);
    let mut out_file = File::create(output_path)?;
    serde_json::to_writer_pretty(&mut out_file, &gb)?;

    return Ok(());
}

fn main() -> io::Result<()> {
    let dir_path = "TE-plasmids";
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            println!("Processing file: {:?}", path);
            if let Err(err) = process_file(&path) {
                eprintln!("Error processing {:?}: {}", path, err);
            }
        }
    }

    return Ok(());
}
