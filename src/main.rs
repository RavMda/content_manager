use std::{
	error::Error,
	fs,
	path::{Path, PathBuf},
};

mod parser;
mod tests;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
	let input_path = Path::new("input");
	let output_path = Path::new("output");

	let addon_packs: Vec<PathBuf> = fs::read_dir(input_path)?
		.into_iter()
		.filter_map(|entry| entry.ok())
		.map(|entry| entry.path())
		.filter(|entry| entry.is_dir())
		.collect();

	// create same folders in ./output

	if output_path.exists() {
		fs::remove_dir_all(output_path)?;
	}

	fs::create_dir(output_path)?;

	for entry in addon_packs.iter() {
		let entry_name = entry
			.file_name()
			.ok_or("failed to get file name")?
			.to_str()
			.ok_or("failed to convert file name to string")?;

		fs::create_dir(output_path.join(entry_name))?;
	}

	// create _lua additionally
	fs::create_dir(output_path.join("_lua"))?;

	Ok(())
}
