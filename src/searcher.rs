use std::fs;
use std::io::{self, prelude::*};
use std::path;

use super::config;
use super::matcher;

use regex::Regex;

const EXTENTIONS: [&str; 10] = [".py",  ".md", ".rst", ".rs", ".js", ".html", ".txt", ".c", ".tf", ".tfstate"];

struct Stats {
    seen: u32,
    skipped: u32,
    analyzed: u32,
}

impl Stats {
    fn new() -> Self {
        Self {
            analyzed: 0,
            skipped: 0,
            seen: 0,
        }
    }
}

pub fn run_search(config: &config::Config) -> io::Result<Vec<matcher::Match>> {
    // Might be usefull to debug someday so I keep it commented
    // if config.verbose {
    //     println!("{}\n", config);
    // }

    let mut stats: Stats = Stats::new();
    let results = search_dir(&config.dir, &config, &mut stats).unwrap_or_default();

    if config.verbose {
        println!(
            "statitics:
    analyzed:       {}
    skipped:        {}
    not readable:   {}
    total matches:  {}\n",
            stats.analyzed,
            stats.skipped,
            stats.seen - stats.analyzed - stats.skipped,
            results.iter().map(|x| x.count).sum::<u32>(),
        );
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

        if !config.hidden && file.file_name().to_str().unwrap().starts_with(".") {
            continue;
        }

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
    stats.seen += 1;

    if !config.all_files {
        if !EXTENTIONS
            .iter()
            .any(|x| filename.file_name().unwrap().to_str().unwrap().ends_with(x))
        {
            stats.skipped += 1;
            return Ok(Vec::new());
        }
    }

    let file = fs::File::open(filename)?;
    let meta = file.metadata()?;

    if !meta.is_file() {
        return Ok(Vec::new());
    }

    let reader = io::BufReader::new(file);
    let mut results: Vec<matcher::Match> = Vec::new();

    stats.analyzed += 1;

    let mut empty_count = 0;
    for (row_nb, mut row) in reader.lines().map(|x| x.unwrap_or_default()).enumerate() {
        // ensure we skip file if we have "infinite" amount of empty rows
        if row.is_empty() {
            empty_count += 1;
            if empty_count > 5 {
                break;
            }
            continue;
        } else {
            empty_count = 0;
        }

        if !config.case_sensitive {
            row = row.to_lowercase();
        }

        if config.exclude.iter().any(|w| row.contains(w)) {
            continue;
        }

        for word in &config.words {
            let re = Regex::new(word).unwrap();
            let count = re.captures_iter(row.as_str()).count();
            if count > 0 {
                results.push(matcher::Match {
                    count: count as u32,
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
