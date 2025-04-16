use std::fs;

use sgdna_lib::objects::config::Config;
use sgdna_lib::objects::gb_container::GBContainer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = "test_data";

    let mut promoters: Vec<String> = vec![];
    let mut terminators: Vec<String> = vec![];
    let mut rbs: Vec<String> = vec![];
    let mut rep_origins: Vec<String> = vec![];

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if ext != "json" {
                    continue;
                }
            }

            let json_string = fs::read_to_string(&path)?;
            let gb: GBContainer = serde_json::from_str(&json_string)?;

            for feature in gb.features {
                match feature.name.as_str() {
                    "promoter" => promoters.push(
                        gb.dna.clone()[(feature.start as usize)..(feature.end as usize)]
                            .to_string(),
                    ),
                    "terminator" => terminators.push(
                        gb.dna.clone()[(feature.start as usize)..(feature.end as usize)]
                            .to_string(),
                    ),
                    "RBS" => rbs.push(
                        gb.dna.clone()[(feature.start as usize)..(feature.end as usize)]
                            .to_string(),
                    ),
                    "rep_origin" => rep_origins.push(
                        gb.dna.clone()[(feature.start as usize)..(feature.end as usize)]
                            .to_string(),
                    ),
                    _ => (),
                }
            }
        }
    }

    promoters.sort();
    terminators.sort();
    rbs.sort();
    rep_origins.sort();

    promoters.dedup();
    terminators.dedup();
    rbs.dedup();
    rep_origins.dedup();

    let config = Config {
        promoters,
        terminators,
        rbs,
        rep_origins,
    };

    let mut out_file = fs::File::create("config.json")?;
    serde_json::to_writer_pretty(&mut out_file, &config)?;

    return Ok(());
}
