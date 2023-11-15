use std::error::Error;

mod content_parser;
mod source_parser;
mod tests;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
	content_parser::run()?;

	Ok(())
}
