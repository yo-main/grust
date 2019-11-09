use std::fs;
use std::io::{self, prelude::*};
use std::path;

use super::config;
use super::matcher;

pub fn run_search(config: &config::Config) -> io::Result<Vec<matcher::Match>> {
    if config.verbose {
        println!("{}\n", config);
    }

    let results = search_dir(&config.dir, &config).unwrap_or_default();

    if config.verbose {
        println!("{:?}", results);
    };
    println!("{} references found\n", results.iter().map(|x| x.count).sum::<u32>());
    Ok(results)
}

fn search_dir(dir: &path::PathBuf, config: &config::Config) -> io::Result<Vec<matcher::Match>> {
    let mut results: Vec<matcher::Match> = Vec::new();

    for path in fs::read_dir(dir)? {
        let file = path?;
        let metadata = file.metadata()?;

        if metadata.is_dir() {
            if config.recursive {
                results = [
                    results,
                    search_dir(&file.path(), &config).unwrap_or_default(),
                ]
                .concat();
            }
        } else {
            results = [
                results,
                search_file(&file.path(), &config).unwrap_or_default(),
            ]
            .concat();
        }
    }
    Ok(results)
}

fn search_file(
    filename: &path::PathBuf,
    config: &config::Config,
) -> io::Result<Vec<matcher::Match>> {
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut results: Vec<matcher::Match> = Vec::new();

    for (row_nb, mut row) in reader.lines().map(|x| x.unwrap_or_default()).enumerate() {
        if !config.case_sensitive {
            row = row.to_lowercase();
        }

        if config.exclude.iter().any(|w| row.contains(w)) {
            continue;
        }

        for word in &config.words {
            if row.contains(word.as_str()) {
                results.push(matcher::Match {
                    count: row.split(word.as_str()).count() as u32 - 1,
                    data: row.trim().to_string(),
                    word: word.clone(),
                    row: row_nb as u32,
                    file: path::PathBuf::from(filename),
                })
            }
        }
    }
    Ok(results)
}
