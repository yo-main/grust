mod arguments;
mod config;
mod matcher;
mod searcher;

pub fn process() {
    let config = prepare_config();

    if config.help {
        // display_help();
    } else {
        searcher::run_search(config);
    }
}

pub fn prepare_config() -> config::Config {
    let args: Vec<arguments::Argument> = arguments::generate();
    config::Config::new(&args)
}
