use std::io;

mod arguments;
mod config;
mod matcher;
mod searcher;
mod display;

pub fn process() -> Result<(), io::Error>{
    let config = prepare_config();

    if config.help {
        // display_help();
    } else {
        let results = searcher::run_search(&config)?;

        if results.is_empty() {
            println!("No results have been found !");
        }

        display::display_results(&results, &config);
    }

    Ok(())
}

pub fn prepare_config() -> config::Config {
    let args: Vec<arguments::Argument> = arguments::generate();
    config::Config::new(&args)
}
