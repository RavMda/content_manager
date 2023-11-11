mod parser;

fn main() {
	let file = std::fs::read("./models/fence.mdl").expect("error opening models/fence.mdl");
	let parsed = parser::parse_model(file);

	println!("directories: {:?}\ntextures: {:?}", parsed.directories, parsed.textures);
}
