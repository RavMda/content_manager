mod parser;
mod tests;

fn main() {
	let file = std::fs::read("./src/tests/models/keycard.mdl").expect("error opening models/fence.mdl");
	let parsed = match parser::parse_model(&file) {
		Ok(parsed) => parsed,
		Err(err) => panic!("{}", err),
	};

	println!(
		"directories: {:?}\ntextures: {:?}",
		parsed.directories, parsed.textures
	);
}
