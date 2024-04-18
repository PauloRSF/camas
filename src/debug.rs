use std::error::Error;

use log::debug;
use owo_colors::OwoColorize;

pub fn log(tag: &str, message: &String) -> Result<(), Box<dyn Error>> {
    debug!("{} {}: {:?}", "[camas]".yellow(), tag.bold(), message);

    Ok(())
}
