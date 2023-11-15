use std::{
	error::Error,
	fs,
	path::{Path, PathBuf},
};

use walkdir::WalkDir;

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

#[derive(serde::Deserialize, Debug)]
struct Config {
	input_folder: String,
	output_folder: String,
	ignored_addon_packs: Vec<String>,
	model_whitelist: bool,
}

fn main() -> Result<()> {
	let config_file = fs::read_to_string("Config.toml")?;
	let config: Config = toml::from_str(&config_file)?;

	let input_path = Path::new(&config.input_folder);
	let output_path = Path::new(&config.output_folder);

	if output_path.exists() {
		fs::remove_dir_all(output_path)?;
	}

	fs::create_dir(output_path)?;
	fs::create_dir(output_path.join("_lua"))?;

	let addon_packs = get_folders(input_path)?;

	let addon_packs: Vec<String> = addon_packs
		.iter()
		.map(|f| {
			f.file_stem()
				.expect("failed to get file stem")
				.to_string_lossy()
				.into()
		})
		.collect();

	for addon_pack in addon_packs {
		if config.ignored_addon_packs.contains(&addon_pack) {
			continue;
		}

		let addon_pack_path = input_path.join(&addon_pack);
		let mut using_whitelist = false;
		let model_whitelist_path = addon_pack_path.join("models.json");

		let mut model_whitelist: Vec<PathBuf> = vec![];

		if config.model_whitelist && model_whitelist_path.exists() {
			let model_whitelist_file = fs::read_to_string(model_whitelist_path)?;
			let model_whitelist_json: Vec<PathBuf> = serde_json::from_str(&model_whitelist_file)?;
			using_whitelist = true;

			for model_path in model_whitelist_json {
				let model_stem: PathBuf = model_path.file_stem().unwrap_or_default().into();

				model_whitelist.push(model_stem);
			}
		}

		let addons = fs::read_dir(&addon_pack_path)?
			.flatten()
			.filter(|f| f.path().is_dir());

		let mut used_materials: Vec<PathBuf> = vec![];

		for addon in addons {
			WalkDir::new(addon.path())
				.sort_by(|a, b| {
					let a_is_model_folder = a.path().ends_with("models");
					let b_is_model_folder = b.path().ends_with("models");

					if a_is_model_folder && !b_is_model_folder {
						std::cmp::Ordering::Less
					} else if !a_is_model_folder && b_is_model_folder {
						std::cmp::Ordering::Greater
					} else {
						std::cmp::Ord::cmp(&a.path(), &b.path())
					}
				})
				.into_iter()
				.flatten()
				.filter(|f| f.path().is_file())
				.try_for_each(|f| -> Result<()> {
					let file_stem: PathBuf = f.path().file_stem().unwrap_or_default().into();
					let file_stem = file_stem.with_extension("");

					let file_ext = f
						.path()
						.extension()
						.and_then(|ext| ext.to_str())
						.unwrap_or_default();

					let normalized_path: PathBuf = f.path().components().skip(3).collect();

					let subpath: String = f
						.path()
						.components()
						.nth(3)
						.map(|c| c.as_os_str().to_string_lossy().to_string())
						.unwrap_or_default();

					if subpath == "models" {
						if using_whitelist {
							if !&model_whitelist.contains(&file_stem) {
								return Ok(());
							}

							if file_ext == "mdl" {
								let file = fs::read(f.path())?;

								let parsed_model = crate::source_parser::mdl::parse_model(&file)?;
								used_materials.extend(parsed_model.used_paths);
							}
						}
					}

					if subpath == "materials" {
						if using_whitelist && !used_materials.contains(&normalized_path) {
							return Ok(());
						}
					}

					let mut out_path = output_path.join(&addon_pack).join(&normalized_path);

					if subpath == "lua" {
						let addon_path = addon.path();
						let addon_stem = addon_path
							.file_stem()
							.unwrap();

						out_path = output_path.join("_lua".to_string()).join(addon_stem).join(normalized_path);
					}

					let out_path_folder = out_path.parent().unwrap();


					fs::create_dir_all(&out_path_folder)?;
					fs::copy(f.path(), out_path)?;

					Ok(())
				})?;
		}
	}

	Ok(())
}
