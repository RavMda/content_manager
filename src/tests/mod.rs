#[cfg(test)]
mod tests {
	#[test]
	fn fence_model_parsing_works() {
		let file =
			std::fs::read("./src/tests/models/fence.mdl").expect("error opening models/fence.mdl");

		let parsed = match crate::source_parser::mdl::parse_model(&file) {
			Ok(parsed) => parsed,
			Err(err) => panic!("{}", err),
		};

		println!("{:?}", parsed);

		assert_eq!(parsed.directories.len(), 1);
		assert_eq!(parsed.directories[0], "models\\_holly\\");

		assert_eq!(parsed.textures.len(), 2);
		assert_eq!(parsed.textures[0], "FencePosts");
		assert_eq!(parsed.textures[1], "FencePlanks");
	}

	#[test]
	fn keycard_model_parsing_works() {
		let file = std::fs::read("./src/tests/models/keycard.mdl")
			.expect("error opening models/keycard.mdl");

		let parsed = match crate::source_parser::mdl::parse_model(&file) {
			Ok(parsed) => parsed,
			Err(err) => panic!("{}", err),
		};

		assert_eq!(parsed.directories.len(), 2);
		assert_eq!(parsed.directories[0], "models\\mishka\\props\\");
		assert_eq!(parsed.directories[1], "models\\mishka\\props2\\");
	}

	#[test]
	fn notmodel_parins_fails() {
		let file = std::fs::read("./src/tests/models/notmodel.mdl")
			.expect("error opening models/notmodel.mdl");

		match crate::source_parser::mdl::parse_model(&file) {
			Ok(_) => panic!("didn't fail"),
			Err(_) => return,
		};
	}

	#[test]
	fn fence_model_parsing_benchmark() {
		let file =
			std::fs::read("./src/tests/models/fence.mdl").expect("error opening models/fence.mdl");

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
}
