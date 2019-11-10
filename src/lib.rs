use std::io;

mod arguments;
mod config;
mod display;
mod matcher;
mod searcher;

pub fn process() -> Result<(), io::Error> {
    let config = prepare_config();

    if config.help {
        display::display_help();
    } else {
        let results = searcher::run_search(&config)?;

        if results.is_empty() {
            println!("No results have been found !");
        } else {
            display::display_results(&results, &config);
        }
    }

    Ok(())
}

pub fn prepare_config() -> config::Config {
    let args: Vec<arguments::Argument> = arguments::generate();
    config::Config::new(&args)
}
