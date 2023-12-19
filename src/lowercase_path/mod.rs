use std::path::{Path, PathBuf};

pub trait AsLowercasePath {
	fn to_lowercase(&self) -> PathBuf;
}

impl AsLowercasePath for Path {
	fn to_lowercase(&self) -> PathBuf {
		let lowercase_str = self.to_string_lossy().to_lowercase();
		PathBuf::from(lowercase_str)
	}
}

impl AsLowercasePath for PathBuf {
	fn to_lowercase(&self) -> PathBuf {
		self.as_path().to_lowercase()
	}
}
