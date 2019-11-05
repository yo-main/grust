pub struct Argument {
    pub id: String,
    pub short: String,
    pub long: String,
    pub help: String,
    pub default: DefaultValue,
}

impl Argument {
    pub fn print(&self) {
        println!(
            "short: {}\nlong: {}\n help: {}\n",
            self.short, self.long, self.help
        )
    }
}

pub enum DefaultValue {
    Bool(bool),
    Text(String),
}

pub fn generate() -> Vec<Argument> {
    vec![
        Argument {
            id: String::from("words"),
            short: String::new(),
            long: String::new(),
            help: String::from("Provide one or several words to look for"),
            default: DefaultValue::Text(String::new()),
        },
        Argument {
            id: String::from("help"),
            short: String::from("-h"),
            long: String::from("--help"),
            help: String::from("Show the help menu"),
            default: DefaultValue::Bool(false),
        },
        Argument {
            id: String::from("recursive"),
            short: String::from("-r"),
            long: String::from("--recursive"),
            help: String::from("Look in the given path recursively"),
            default: DefaultValue::Bool(true),
        },
        Argument {
            id: String::from("case_sensitive"),
            short: String::from("-cs"),
            long: String::from("--case_sensitive"),
            help: String::from("Apply case sensitivity to the search"),
            default: DefaultValue::Bool(false),
        },
        Argument {
            id: String::from("verbose"),
            short: String::from("-v"),
            long: String::from("--verbose"),
            help: String::from("Show more detailed about the findings"),
            default: DefaultValue::Bool(false),
        },
        Argument {
            id: String::from("save"),
            short: String::from("-s"),
            long: String::from("--save"),
            help: String::from("Save the results in a file in the current path"),
            default: DefaultValue::Bool(false),
        },
        Argument {
            id: String::from("dir"),
            short: String::from("-d"),
            long: String::from("--dir"),
            help: String::from("Path from where the search will start"),
            default: DefaultValue::Text(String::from(".")),
        },
        Argument {
            id: String::from("full_path"),
            short: String::from("-fp"),
            long: String::from("--full_path"),
            help: String::from("Show the file's full path in the result view"),
            default: DefaultValue::Bool(false),
        },
        Argument {
            id: String::from("exclude"),
            short: String::from("-e"),
            long: String::from("--exclude"),
            help: String::from("Comma-separated list of words. \nRow won't be matched if they include one of those words"),
            default: DefaultValue::Text(String::new()),
        }
    ]
}
