use std::{
	error::Error,
	fs::{self},
	path::{Path, PathBuf},
};

use walkdir::DirEntry;

use walkdir::WalkDir;

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
pub struct Config {
	pub input_folder: String,
	pub output_folder: String,
	pub ignored_addon_packs: Vec<String>,
	pub model_whitelist: bool,
}

fn sort_by_models(a: &DirEntry, b: &DirEntry) -> core::cmp::Ordering {
	let a_is_model_folder = a.path().ends_with("models");
	let b_is_model_folder = b.path().ends_with("models");

	if a_is_model_folder && !b_is_model_folder {
		std::cmp::Ordering::Less
	} else if !a_is_model_folder && b_is_model_folder {
		std::cmp::Ordering::Greater
	} else {
		std::cmp::Ord::cmp(&a.path(), &b.path())
	}
}

fn create_output_directory(output_path: &Path) -> Result<()> {
	if output_path.exists() {
		fs::remove_dir_all(output_path)?;
	}

	fs::create_dir(output_path)?;
	fs::create_dir(output_path.join("_lua"))?;

	Ok(())
}

fn get_addon_packs(input_path: &Path, config: &Config) -> Result<Vec<String>> {
	let addon_packs = get_folders(input_path)?;

	let addon_packs: Vec<String> = addon_packs
		.iter()
		.map(|f| f.file_stem().unwrap_or_default().to_string_lossy().into())
		.collect();

	let filtered_addon_packs: Vec<String> = addon_packs
		.into_iter()
		.filter(|addon_pack| !config.ignored_addon_packs.contains(addon_pack))
		.collect();

	Ok(filtered_addon_packs)
}

pub fn run(config: &Config) -> Result<()> {
	let input_path = Path::new(&config.input_folder);
	let output_path = Path::new(&config.output_folder);

	create_output_directory(output_path)?;

	let addon_packs = get_addon_packs(input_path, &config)?;

	for addon_pack in addon_packs {
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
			.filter(|entry| entry.path().is_dir());

		let mut used_materials: Vec<PathBuf> = vec![];

		for addon in addons {
			WalkDir::new(addon.path())
				.sort_by(sort_by_models)
				.into_iter()
				.flatten()
				.filter(|f| f.path().is_file())
				.try_for_each(|f| -> Result<()> {
					// file stem without extension
					let file_stem: PathBuf = f.path().file_stem().unwrap_or_default().into();
					let file_stem = file_stem.with_extension("");

					let file_extension = f
						.path()
						.extension()
						.and_then(|ext| ext.to_str())
						.unwrap_or_default();

					let clear_path = f.path().strip_prefix(&input_path)?;

					let normalized_path: PathBuf = clear_path.components().skip(2).collect();

					// e.g. materials, models, sounds, etc
					let subpath: String = clear_path
						.iter()
						.nth(2)
						.map(|c| c.to_string_lossy().to_string())
						.unwrap_or_default();

					if subpath == "models" {
						if using_whitelist {
							if !&model_whitelist.contains(&file_stem) {
								return Ok(());
							}

							if file_extension == "mdl" {
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
						let addon_stem = addon_path.file_stem().unwrap();

						out_path = output_path
							.join("_lua".to_string())
							.join(addon_stem)
							.join(&normalized_path);
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
