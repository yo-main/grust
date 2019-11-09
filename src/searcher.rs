use std::fs;
use std::io::{self, prelude::*};
use std::path;

use super::config;
use super::matcher;

const extentions: [&str; 6] = [".py", ".rs", ".js", ".html", ".txt", ".c"];

struct Stats {
    seen: u32,
    found: u32,
    skipped: u32,
}

impl Stats {
    fn new() -> Self {
        Self {
            found: 0,
            skipped: 0,
            seen: 0,
        }
    }
}

pub fn run_search(config: &config::Config) -> io::Result<Vec<matcher::Match>> {
    if config.verbose {
        println!("{}\n", config);
    }

    let mut stats: Stats = Stats::new();
    let results = search_dir(&config.dir, &config, &mut stats).unwrap_or_default();
    println!("\n");
    println!(
        "{} analyzed for {} matches\n",
        stats.seen - stats.skipped,
        results.iter().map(|x| x.count).sum::<u32>()
    );

    if config.verbose {
        println!("{:?}", results);
    };
    Ok(results)
}

fn search_dir(
    dir: &path::PathBuf,
    config: &config::Config,
    mut stats: &mut Stats,
) -> io::Result<Vec<matcher::Match>> {
    let mut results: Vec<matcher::Match> = Vec::new();

    for path in fs::read_dir(dir)? {
        let file = path?;
        let metadata = file.metadata()?;

        if metadata.is_dir() {
            if config.recursive {
                results = [
                    results,
                    search_dir(&file.path(), &config, &mut stats).unwrap_or_default(),
                ]
                .concat();
            }
        } else {
            results = [
                results,
                search_file(&file.path(), &config, &mut stats).unwrap_or_default(),
            ]
            .concat();
        }
    }
    Ok(results)
}

fn search_file(
    filename: &path::PathBuf,
    config: &config::Config,
    stats: &mut Stats,
) -> io::Result<Vec<matcher::Match>> {
    print!("\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r");
    print!("{} file(s) seen", stats.seen);
    stats.seen += 1;

    if !config.all_files {
        if !extentions
            .iter()
            .any(|x| filename.file_name().unwrap().to_str().unwrap().ends_with(x))
        {
            stats.skipped += 1;
            return Ok(Vec::new());
        }
    }

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
                stats.found += 1;
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
