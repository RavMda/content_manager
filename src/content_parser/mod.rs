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
	pub input_folder: PathBuf,
	pub output_folder: PathBuf,
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

struct AddonPack {
	name: String,
	whitelist: bool,
	used_models: Vec<PathBuf>,
	used_materials: Vec<PathBuf>,
}

impl AddonPack {
	fn load_whitelist(&mut self, config: &Config) -> Result<()> {
		if !config.model_whitelist {
			return Ok(());
		}

		let whitelist_path = config.input_folder.join(&self.name).join("models.json");

		if !whitelist_path.exists() {
			return Ok(());
		}

		let whitelist_file = fs::read_to_string(whitelist_path)?;
		let whitelist_json: Vec<PathBuf> = serde_json::from_str(&whitelist_file)?;

		let whitelist: Vec<PathBuf> = whitelist_json
			.into_iter()
			.map(|model_path| model_path.file_stem().unwrap_or_default().into())
			.collect();

		self.used_models = whitelist;
		self.whitelist = true;

		Ok(())
	}

	fn uses_model(&self, model: &PathBuf) -> bool {
		self.used_models.contains(model)
	}

	fn uses_material(&self, material: &PathBuf) -> bool {
		self.used_materials.contains(material)
	}
}

fn process_addon(addon: &fs::DirEntry, config: &Config, addon_pack: &mut AddonPack) -> Result<()> {
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

			let clear_path = f.path().strip_prefix(&config.input_folder)?;
			let normalized_path: PathBuf = clear_path.components().skip(2).collect();

			// e.g. "materials", "models", "sounds", etc
			let subpath: String = clear_path
				.iter()
				.nth(2)
				.map(|c| c.to_string_lossy().to_string())
				.unwrap_or_default();

			let output_folder = &config.output_folder;
			let mut out_path = output_folder.join(&addon_pack.name).join(&normalized_path);

			match subpath.as_str() {
				"models" => {
					if addon_pack.whitelist && !addon_pack.uses_model(&file_stem) {
						return Ok(());
					}

					if addon_pack.whitelist && file_extension == "mdl" {
						let file = fs::read(f.path())?;
						let parsed_model = crate::source_parser::mdl::parse_model(&file)?;

						addon_pack.used_materials.extend(parsed_model.used_paths);
					}
				}
				"materials" => {
					if addon_pack.whitelist && !addon_pack.uses_material(&normalized_path) {
						return Ok(());
					}
				}
				"lua" => {
					let addon_path = addon.path();
					let addon_stem = addon_path.file_stem().unwrap();

					out_path = output_folder
						.join("_lua".to_string())
						.join(addon_stem)
						.join(&normalized_path);
				}
				_ => {}
			}

			fs::create_dir_all(out_path.parent().ok_or("couldn't get out_path parent")?)?;
			fs::copy(f.path(), out_path)?;

			Ok(())
		})?;

	Ok(())
}

pub fn run(config: &Config) -> Result<()> {
	create_output_directory(&config.output_folder)?;

	let addon_packs = get_addon_packs(&config.input_folder, &config)?;

	for addon_pack_name in addon_packs {
		println!("processing \"{}\":", addon_pack_name);

		let addon_pack_path = &config.input_folder.join(&addon_pack_name);

		let mut addon_pack = AddonPack {
			name: addon_pack_name,
			used_models: vec![],
			used_materials: vec![],
			whitelist: false,
		};

		addon_pack.load_whitelist(config)?;

		let addons = fs::read_dir(&addon_pack_path)?
			.flatten()
			.filter(|entry| entry.path().is_dir());

		for addon in addons {
			let addon_name = addon.file_name();

			println!("- {}", addon_name.to_string_lossy());

			process_addon(&addon, &config, &mut addon_pack)?;
		}
	}

	Ok(())
}
