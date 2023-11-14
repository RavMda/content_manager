use std::{
	error::Error,
	fs,
	path::{Path, PathBuf},
};

mod source_parser;
mod tests;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn get_folders(path: &Path) -> Result<Vec<PathBuf>> {
	let folders: Vec<PathBuf> = fs::read_dir(path)?
		.into_iter()
		.filter_map(|entry| entry.ok())
		.map(|entry| entry.path())
		.filter(|entry| entry.is_dir())
		.collect();

	Ok(folders)
}

fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<()> {
	fs::create_dir_all(&destination)?;

	for entry in fs::read_dir(source)? {
		let entry = entry?;
		let is_dir = entry.file_type()?.is_dir();

		if is_dir {
			copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
		} else {
			fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
		}
	}

	Ok(())
}

#[derive(serde::Deserialize, Debug)]
struct Config {
	input_folder: String,
	output_folder: String,
	ignored_addon_packs: Vec<String>,
	model_whitelist: ModelWhitelist,
}

#[derive(serde::Deserialize, Debug)]
struct ModelWhitelist {
	enabled: bool,
	path: String,
}

fn main() -> Result<()> {
	let config_file = fs::read_to_string("Config.toml")?;
	let config: Config = toml::from_str(&config_file)?;

	let input_path = Path::new(&config.input_folder);
	let output_path = Path::new(&config.output_folder);

	let addon_packs = get_folders(input_path)?;

	if output_path.exists() {
		fs::remove_dir_all(output_path)?;
	}

	fs::create_dir(output_path)?;
	fs::create_dir(output_path.join("_lua"))?;

	addon_packs
		.iter()
		.flat_map(|addon_pack| fs::read_dir(addon_pack).unwrap().flatten())
		.try_for_each(|addon| -> Result<()> {
			let addon_path = addon.path();
			let addon_subfolders = get_folders(addon_path.as_path());

			let addon_stem = addon_path.file_stem().ok_or("couldn't get addon_stem")?;
			let addon_pack = addon_path.parent().ok_or("couldn't get addon_pack")?;
			let addon_pack_stem = addon_pack
				.file_stem()
				.ok_or("couldn't get addon_pack_stem")?;

			let addon_pack_stem_str = addon_pack_stem
				.to_str()
				.ok_or("couldn't convert addon_pack_stem to string")?
				.to_string();

			if config.ignored_addon_packs.contains(&addon_pack_stem_str) {
				return Ok(());
			}

			for subfolder in addon_subfolders.iter().flatten() {
				let subfolder_stem = subfolder.file_stem().ok_or("couldn't get subfolder stem")?;
				let mut final_folder = output_path.join(addon_pack_stem).join(subfolder_stem);

				if subfolder_stem == "lua" {
					final_folder = output_path.join("_lua").join(addon_stem);
				}

				fs::create_dir_all(&final_folder)?;
				copy_recursively(subfolder, &final_folder)?;
			}

			Ok(())
		})?;

	Ok(())
}
