use byteorder::{LittleEndian, ReadBytesExt};
use core::fmt;
use io::Read;
use std::{error::Error, io};

#[derive(Debug)]
pub struct ParsedModel {
	pub directories: Vec<String>,
	pub textures: Vec<String>,
	pub used_paths: Vec<String>,
}

struct ModelReader<'a> {
	reader: std::io::Cursor<&'a Vec<u8>>,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

impl<'a> ModelReader<'a> {
	fn read_string(&mut self, size: usize) -> Result<String> {
		let mut string_vec = vec![0u8; size];

		self.reader.read_exact(&mut string_vec)?;

		let string = String::from_utf8(string_vec)?;
		Ok(string)
	}

	fn read_int(&mut self) -> Result<i32> {
		let int = self.reader.read_i32::<LittleEndian>()?;
		Ok(int)
	}

	#[allow(dead_code)]
	fn read_vector(&mut self) -> Result<Vec<f32>> {
		let x = self.reader.read_f32::<LittleEndian>()?;
		let y = self.reader.read_f32::<LittleEndian>()?;
		let z = self.reader.read_f32::<LittleEndian>()?;

		return Ok(vec![x, y, z]);
	}

	fn read_byte(&mut self) -> Result<u8> {
		let mut byte_buf: [u8; 1] = [0];

		self.reader.read(&mut byte_buf)?;

		return Ok(byte_buf[0]);
	}

	fn read_c_str(&mut self) -> Result<String> {
		let mut string_vec: Vec<u8> = vec![];

		loop {
			let byte = self.read_byte()?;

			if byte == 0 {
				break;
			}

			string_vec.push(byte);
		}

		let string = String::from_utf8(string_vec)?;
		Ok(string)
	}

	fn set_pos(&mut self, pos: i32) {
		self.reader.set_position(pos as u64);
	}

	fn skip(&mut self, to_skip: u64) {
		self.reader.set_position(self.reader.position() + to_skip);
	}
}

#[derive(Debug)]
pub struct ErrorModelFormat(String);

impl fmt::Display for ErrorModelFormat {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "incorrect model format! {}", self.0)
	}
}

impl std::error::Error for ErrorModelFormat {}

pub fn parse_model(file: &Vec<u8>) -> Result<ParsedModel> {
	let mut model_textures = ParsedModel {
		directories: vec![],
		textures: vec![],
		used_paths: vec![],
	};

	let mut model_reader = ModelReader {
		reader: std::io::Cursor::new(&file),
	};

	let model_format = model_reader.read_string(4)?;

	if model_format != "IDST" {
		return Err(Box::new(ErrorModelFormat(model_format)));
	};

	model_reader.skip(200);

	let texture_count = model_reader.read_int()?;
	let texture_offset = model_reader.read_int()?;
	let texturedir_count = model_reader.read_int()?;
	let texturedir_offset = model_reader.read_int()?;

	model_reader.set_pos(texturedir_offset);

	let texturedir_string_offset = model_reader.read_int()?;
	model_reader.set_pos(texturedir_string_offset);

	for _ in 0..texturedir_count {
		let dir = model_reader.read_c_str()?;
		model_textures.directories.push(dir);
	}

	model_reader.set_pos(texture_offset);

	let texture_path_offset = model_reader.read_int()? + texture_offset;
	model_reader.set_pos(texture_path_offset);

	for _ in 0..texture_count {
		let texture = model_reader.read_c_str()?;
		model_textures.textures.push(texture);
	}

	return Ok(model_textures);
}
