use super::arguments;
use std::env;
use std::path;

pub struct Config {
    pub dir: path::PathBuf,
    pub help: bool,
    pub recursive: bool,
    pub words: Vec<String>,
    pub exclude: Vec<String>,
    pub case_sensitive: bool,
    pub verbose: bool,
    pub save_result: bool,
    pub full_path: bool,
}

impl Config {
    pub fn new(args: &Vec<arguments::Argument>) -> Self {
        let mut out = Self {
            words: get_vector_from("words", &args),
            help: get_bool_from("help", &args),
            dir: path::PathBuf::from(get_string_from("dir", &args)),
            recursive: get_bool_from("recursive", &args),
            exclude: get_vector_from("exclude", &args),
            case_sensitive: get_bool_from("case_sensitive", &args),
            verbose: get_bool_from("verbose", &args),
            full_path: get_bool_from("full_path", &args),
            save_result: get_bool_from("save", &args),
        };
        out.update();
        out
    }

    pub fn update(&mut self) {
        let args = env::args().collect::<Vec<String>>();
        let mut iterator = args.iter();
        iterator.next(); // get rid of the first argument (the script itself)

        loop {
            let arg = match iterator.next() {
                None => break,
                Some(i) => i,
            };

            match arg.as_str() {
                "-h" => self.help = true,
                "--help" => self.help = true,
                "-r" => self.recursive = true,
                "--recursive" => self.recursive = true,
                "-v" => self.verbose = true,
                "--verbose" => self.verbose = true,
                "-cs" => self.case_sensitive = true,
                "--case_sensitive" => self.case_sensitive = true,
                "-s" => self.save_result = true,
                "--save" => self.save_result = true,
                "-fp" => self.full_path = true,
                "--full_path" => self.full_path = true,
                "-e" => {
                    self.exclude = iterator
                        .next()
                        .cloned()
                        .unwrap_or_default()
                        .split(',')
                        .map(String::from)
                        .collect()
                }
                "--exclude" => {
                    self.exclude = iterator
                        .next()
                        .cloned()
                        .unwrap_or_default()
                        .split(",")
                        .map(String::from)
                        .collect()
                }
                "-d" => {
                    self.dir =
                        path::PathBuf::from(iterator.next().cloned().unwrap_or(String::from(".")))
                }
                "--dir" => {
                    self.dir =
                        path::PathBuf::from(iterator.next().cloned().unwrap_or(String::from(".")))
                }
                _ => self.words.push(String::from(arg)),
            }
        }

        if !self.case_sensitive {
            self.words = self.words.iter().map(|x| x.to_lowercase()).collect();
            self.exclude = self.exclude.iter().map(|x| x.to_lowercase()).collect();
        }

    }
}

fn get_string_from(id: &str, list: &Vec<arguments::Argument>) -> String {
    let argument = list.iter().find(|x| x.id == id).unwrap();
    match &argument.default {
        arguments::DefaultValue::Text(value) => String::from(value),
        _ => panic!("The string argument {} could not be processed.", id),
    }
}

fn get_vector_from(id: &str, list: &Vec<arguments::Argument>) -> Vec<String> {
    let string = get_string_from(&id, &list);
    match string.is_empty() {
        true => Vec::new(),
        false => string.split(',').map(String::from).collect(),
    }
}

fn get_bool_from(id: &str, list: &Vec<arguments::Argument>) -> bool {
    let argument = list.iter().find(|x| x.id == id).unwrap();
    match &argument.default {
        arguments::DefaultValue::Bool(value) => value.clone(),
        _ => panic!("The bool argument {} could not be processed.", id),
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Configuration object:\n \
             words           -> {:?}\n \
             exclude         -> {:?}\n \
             dir             -> {:?}\n \
             help            -> {}\n \
             recursive       -> {}\n \
             verbose         -> {}\n \
             save            -> {}\n \
             full_path       -> {}\n \
             case_sensitive  -> {}\n \
             ",
            self.words,
            self.exclude,
            self.dir,
            self.help,
            self.recursive,
            self.verbose,
            self.save_result,
            self.full_path,
            self.case_sensitive
        )
    }
}
