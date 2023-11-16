#[cfg(test)]
mod tests {
	use std::{path::Path, vec};

	macro_rules! assert_file {
		($addon_pack:expr, $path_str:expr, $exists:expr) => {{
			let path = $addon_pack.join($path_str);
			assert_eq!(
				std::fs::metadata(&path).is_ok(),
				$exists,
				"file does not exist: {:?}",
				path
			);
		}};
	}

	macro_rules! assert_model {
		($addon_pack:expr, $model_name:expr, $exists:expr) => {{
			let model_path = format!("{}.mdl", $model_name);
			let phy_path = format!("{}.phy", $model_name);
			let vvd_path = format!("{}.vvd", $model_name);
			let sw_vtx_path = format!("{}.sw.vtx", $model_name);
			let dx80_vtx_path = format!("{}.dx80.vtx", $model_name);
			let dx90_vtx_path = format!("{}.dx90.vtx", $model_name);

			assert_file!($addon_pack, model_path, $exists);
			assert_file!($addon_pack, phy_path, $exists);
			assert_file!($addon_pack, vvd_path, $exists);
			assert_file!($addon_pack, sw_vtx_path, $exists);
			assert_file!($addon_pack, dx80_vtx_path, $exists);
			assert_file!($addon_pack, dx90_vtx_path, $exists);
		}};
	}

	macro_rules! assert_texture {
		($addon_pack:expr, $texture_name:expr, $exists:expr) => {{
			for i in 1..3 {
				let vtf_path = format!("{}{}.vtf", $texture_name, i);
				let vmt_path = format!("{}{}.vmt", $texture_name, i);

				assert_file!($addon_pack, vtf_path, $exists);
				assert_file!($addon_pack, vmt_path, $exists);
			}
		}};
	}

	#[test]
	fn fence_model_parsing_works() {
		let file = std::fs::read("./src/tests/input/addon_pack_1/addon_1/models/test1.mdl")
			.expect("error opening test1.mdl");

		let parsed = match crate::source_parser::mdl::parse_model(&file) {
			Ok(parsed) => parsed,
			Err(err) => panic!("{}", err),
		};

		assert_eq!(parsed.directories.len(), 1);
		assert_eq!(parsed.directories[0], "test1\\");

		assert_eq!(parsed.textures.len(), 3);
		assert_eq!(parsed.textures[0], "test1_mat1");
		assert_eq!(parsed.textures[1], "test1_mat2");
		assert_eq!(parsed.textures[2], "test1_mat3");
	}

	#[test]
	fn keycard_model_parsing_works() {
		let file = std::fs::read("./src/tests/input/addon_pack_1/addon_1/models/test2.mdl")
			.expect("error opening test2.mdl");

		let parsed = match crate::source_parser::mdl::parse_model(&file) {
			Ok(parsed) => parsed,
			Err(err) => panic!("{}", err),
		};

		assert_eq!(parsed.directories.len(), 2);
		assert_eq!(parsed.directories[0], "test2\\");
		assert_eq!(parsed.directories[1], "test2_2\\");

		assert_eq!(parsed.textures.len(), 3);
		assert_eq!(parsed.textures[0], "test2_mat1");
		assert_eq!(parsed.textures[1], "test2_mat2");
		assert_eq!(parsed.textures[2], "test2_mat3");
	}

	#[test]
	fn notmodel_parsing_fails() {
		let file = std::fs::read("./src/tests/input/addon_pack_1/addon_1/models/test3.mdl")
			.expect("error opening test3.mdl");

		match crate::source_parser::mdl::parse_model(&file) {
			Ok(_) => panic!("didn't fail"),
			Err(_) => return,
		};
	}

	#[test]
	fn fence_model_parsing_benchmark() {
		let file = std::fs::read("./src/tests/input/addon_pack_1/addon_1/models/test1.mdl")
			.expect("error opening test1.mdl");

		use std::time::Instant;
		let now = Instant::now();

		for _ in 0..1000 {
			let _ = match crate::source_parser::mdl::parse_model(&file) {
				Ok(parsed) => parsed,
				Err(err) => panic!("{}", err),
			};
		}

		let elapsed = now.elapsed();
		println!("elapsed: {:.2?}", elapsed);
	}

	#[test]
	fn content_manager_no_whitelist() {
		let config = crate::content_parser::Config {
			input_folder: "./src/tests/input".into(),
			output_folder: "./src/tests/output".into(),
			ignored_addon_packs: vec![],
			model_whitelist: false,
		};

		match crate::content_parser::run(&config) {
			Ok(_) => {}
			Err(err) => {
				panic!("parsing failed: {}", err)
			}
		};

		let addon_pack = Path::new(&config.output_folder).join("addon_pack_1");

		assert_model!(addon_pack, "models/test1", true);
		assert_model!(addon_pack, "models/test2", true);
		assert_model!(addon_pack, "models/test3", true);

		assert_texture!(addon_pack, "materials/test1/test1_mat", true);
		assert_texture!(addon_pack, "materials/test2/test2_mat", true);
		assert_texture!(addon_pack, "materials/test2_2/test2_mat", true);
	}

	#[test]
	fn content_manager_whitelist() {
		let config = crate::content_parser::Config {
			input_folder: "./src/tests/input".into(),
			output_folder: "./src/tests/output".into(),
			ignored_addon_packs: vec![],
			model_whitelist: true,
		};

		match crate::content_parser::run(&config) {
			Ok(_) => {}
			Err(err) => {
				panic!("parsing failed: {}", err)
			}
		};

		let addon_pack = Path::new(&config.output_folder).join("addon_pack_1");

		assert_model!(addon_pack, "models/test1", true);
		assert_model!(addon_pack, "models/test2", false);
		assert_model!(addon_pack, "models/test3", false);

		assert_texture!(addon_pack, "materials/test1/test1_mat", true);
		assert_texture!(addon_pack, "materials/test2/test2_mat", false);
		assert_texture!(addon_pack, "materials/test2_2/test2_mat", false);
	}

	#[test]
	fn content_manager_ignoring() {
		let config = crate::content_parser::Config {
			input_folder: "./src/tests/input".into(),
			output_folder: "./src/tests/output".into(),
			ignored_addon_packs: vec!["addon_pack_1".into()],
			model_whitelist: true,
		};

		match crate::content_parser::run(&config) {
			Ok(_) => {}
			Err(err) => {
				panic!("parsing failed: {}", err)
			}
		};

		let addon_pack = Path::new(&config.output_folder).join("addon_pack_1");

		assert_model!(addon_pack, "models/test1", false);
		assert_model!(addon_pack, "models/test2", false);
		assert_model!(addon_pack, "models/test3", false);

		assert_texture!(addon_pack, "materials/test1/test1_mat", false);
		assert_texture!(addon_pack, "materials/test2/test2_mat", false);
		assert_texture!(addon_pack, "materials/test2_2/test2_mat", false);
	}
}
