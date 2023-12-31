use std::{error::Error, fs};

mod lowercase_path;
mod content_parser;
mod source_parser;
mod tests;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
	let config_file = fs::read_to_string("Config.toml")?;
	let config: content_parser::Config = toml::from_str(&config_file)?;

	content_parser::run(&config)?;

	println!("\npress enter to exit...");
	std::io::stdin().read_line(&mut String::new())?;

	Ok(())
}
