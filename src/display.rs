use std::collections::HashMap;
use std::path;
use std::io;
use std::cmp;

use super::matcher;
use super::config;


struct FileSet {
    words: HashMap::<String, u32>,
    count: u32
}

impl FileSet {
    fn from(words: HashMap::<String, u32>) -> Self {
        Self {
            words: words,
            count: 0
        }
    }
}


pub fn display_results(results: &Vec<matcher::Match>, config: &config::Config) {
    let to_display = parse_results(&results, &config);
    print_results(&to_display, &config);
}

fn print_results(dataset: &HashMap::<path::PathBuf, FileSet>, config: &config::Config) -> io::Result<()> {
    let mut words: Vec<(String, u32)> = config.words.iter().map(|word| 
        (String::from(word), dataset.iter().map(|(_, fileset)| *fileset.words.get(word).unwrap()).collect::<Vec<u32>>().iter().sum::<u32>())
    ).collect();

    words.sort_by(|a, b| b.1.cmp(&a.1));
    
    let filename_max_size = dataset.iter().map(|(filename, _)| filename.to_str().unwrap().len()).max().unwrap();

    let mut title = format!("{:<1$}", "file", filename_max_size);
    for word in &words {
        title.push_str(format!(" | {:>1$}", word.0, cmp::max(word.0.len(), format!("{}", word.1).len())).as_str());
    }

    println!("{}", title);

    for data in dataset {
        let filename = data.0;
        let matches = data.1;

        let mut row = format!("{:>1$}", filename.to_str().unwrap(), filename_max_size);
        for word in &words {
            row.push_str(
                format!(" | {:>1$?}", matches.words.get(&word.0).unwrap(), cmp::max(word.0.len(), format!("{}", word.1).len())).as_str()
            )
        }
        println!("{}", row);
    }

    Ok(())
}

fn parse_results(results: &Vec<matcher::Match>, config: &config::Config) -> HashMap::<path::PathBuf, FileSet> {
    let mut dataset = HashMap::new();
    let mut hash_words = HashMap::new();

    for word in config.words.iter() {
        hash_words.insert(word.clone(), 0);
    }

    for filename in results.iter().map(|x| &x.file) {
        dataset.insert(filename.clone(), FileSet::from(hash_words.clone()));
    }

    for res in results.iter() {
        let fileset = dataset.get_mut(&res.file).expect(format!("{:?} was unknown in the dataset", res.file).as_str());
        let count: u32 = *fileset.words.get_mut(res.word.as_str()).unwrap_or(&mut 0) + 1;
        fileset.words.insert(res.word.clone(), count);
        fileset.count += 1;
    }

    dataset
}


