mod parser;

fn main() {
	let file = std::fs::read("./models/notmodel.mdl").expect("error opening models/fence.mdl");
	let parsed = match parser::parse_model(file) {
		Ok(parsed) => parsed,
		Err(err) => panic!("{}", err),
	};

	println!(
		"directories: {:?}\ntextures: {:?}",
		parsed.directories, parsed.textures
	);
}
