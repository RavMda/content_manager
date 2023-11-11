use std::{io::Read, vec};
use byteorder::{ReadBytesExt, LittleEndian};

#[derive(Debug)]
pub struct ParsedModel {
	pub directories: Vec<String>,
	pub textures: Vec<String>
}

struct ModelReader {
	reader: std::io::Cursor<Vec<u8>>
}

impl ModelReader {
	fn read_string(&mut self, size: usize) -> String {
		let mut string_vec = vec![0u8; size];
		self.reader.read_exact(&mut string_vec).unwrap();

		return String::from_utf8(string_vec).unwrap();
	}

	fn read_int(&mut self) -> i32 {
		return self.reader.read_i32::<LittleEndian>().unwrap();
	}

	#[allow(dead_code)]
	fn read_vector(&mut self) -> Vec<f32> {
		let x = self.reader.read_f32::<LittleEndian>().unwrap();
		let y = self.reader.read_f32::<LittleEndian>().unwrap();
		let z = self.reader.read_f32::<LittleEndian>().unwrap();

		return vec![x, y, z]
	}

	fn read_byte(&mut self) -> u8 {
		let mut byte_buf: [u8; 1] = [0];

		self.reader.read(&mut byte_buf).unwrap();

		return byte_buf[0];
	}

	fn read_c_str(&mut self) -> String {
		let mut string_vec: Vec<u8> = vec![];
		
		loop {
			let byte = self.read_byte();

			if byte == 0 {
				break;
			}

			string_vec.push(byte);
		}

		return String::from_utf8(string_vec).unwrap()
	}

	fn set_pos(&mut self, pos: i32) {
		self.reader.set_position(pos as u64);
	}

	fn skip(&mut self, to_skip: u64) {
		self.reader.set_position(self.reader.position() + to_skip);
	}
}

pub fn parse_model(file: Vec<u8>) -> ParsedModel {
	let mut model_textures = ParsedModel {
		directories: vec![],
		textures: vec![]
	};

	let mut model_reader = ModelReader{
		reader: std::io::Cursor::new(file)
	};

	let model_format = model_reader.read_string(4);

	if model_format != "IDST" {
		panic!("incorrect .mdl format! {}", model_format);
	};

	model_reader.skip(200);

	let texture_count = model_reader.read_int();
	let texture_offset = model_reader.read_int();
	let texturedir_count = model_reader.read_int();
	let texturedir_offset = model_reader.read_int();

	model_reader.set_pos(texturedir_offset);

	let texturedir_string_offset = model_reader.read_int();
	model_reader.set_pos(texturedir_string_offset);
	
	for _ in 0 .. texturedir_count {
		let dir = model_reader.read_c_str();
		model_textures.directories.push(dir);
	}

	model_reader.set_pos(texture_offset);

	let texture_path_offset = model_reader.read_int() + texture_offset;
	model_reader.set_pos(texture_path_offset);

	for _ in 0 .. texture_count {
		let texture = model_reader.read_c_str();
		model_textures.textures.push(texture);
	};

	return model_textures;
}