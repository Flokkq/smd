use log::{
	debug,
	error,
	info,
};

use crate::error::Result;
use std::{
	fs::{
		self,
		File,
	},
	io::Write,
	path::PathBuf,
};

/// Trait for serializing objects to strings.
pub trait Serialize {
	fn serialize(&self) -> Result<String>;
}

/// Trait for deserializing objects from strings or files.
pub trait Deserialize: Sized {
	fn deserialize(input: &str) -> Result<Self>;
}

/// Reads the contents of a file into a string.
pub fn read_to_string(path: &PathBuf) -> Result<String> {
	debug!("Attempting to read file: {}", path.display());
	match fs::read_to_string(path) {
		Ok(content) => {
			info!("Successfully read file: {}", path.display());
			Ok(content)
		}
		Err(e) => {
			error!("Failed to read file: {}", path.display());
			Err(e.into())
		}
	}
}

/// Writes the contents of a string to a file.
pub fn write_to_file(path: &PathBuf, content: &str) -> Result<()> {
	let path_str = path.to_string_lossy();
	let line_count = content.lines().count();

	debug!("Attempting to write file: {}", path_str);

	let mut file = File::create(path)?;
	match file.write(content.as_bytes()) {
		Ok(bytes_written) => {
			info!(
				"\"{}\" {}L, {}B written",
				path_str, line_count, bytes_written
			);
			Ok(())
		}
		Err(e) => {
			error!("Failed to write file: {}", path_str);
			Err(e.into())
		}
	}
}

/// Writes bytes to a file.
pub fn write_bytes(path: &PathBuf, bytes: &Vec<u8>) -> Result<()> {
	let path_str = path.to_string_lossy();

	debug!("Attempting to write file: {}", path_str);

	let mut file = File::create(path)?;
	match file.write(bytes) {
		Ok(bytes_written) => {
			info!("\"{}\", {}B written", path_str, bytes_written);
			Ok(())
		}
		Err(e) => {
			error!("Failed to write file: {}", path_str);
			Err(e.into())
		}
	}
}

/// Serialize an object to a string.
pub fn encode_to_string<T>(value: &T) -> Result<String>
where
	T: Serialize,
{
	debug!("Serializing object...");
	match value.serialize() {
		Ok(serialized) => {
			info!("Object serialized successfully.");
			Ok(serialized)
		}
		Err(e) => {
			error!("Serialization failed: {}", e);
			Err(e)
		}
	}
}

/// Deserialize an object from a file.
pub fn decode_from_file<T>(path: PathBuf) -> Result<T>
where
	T: Deserialize,
{
	debug!("Decoding object from file: {}", path.display());
	let content = read_to_string(&path)?;
	match T::deserialize(&content) {
		Ok(deserialized) => {
			info!("Deserialization successful.");
			Ok(deserialized)
		}
		Err(e) => {
			error!("Deserialization failed: {}", e);
			Err(e)
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::error::Error;

	use super::*;

	#[derive(Debug, PartialEq)]
	struct MyStruct {
		name:  String,
		value: i32,
	}

	impl Serialize for MyStruct {
		fn serialize(&self) -> Result<String> {
			if self.name.is_empty() {
				return Err(Error::SerializeError(
					"Name cannot be empty".to_string(),
				));
			}
			Ok(format!("{}:{}", self.name, self.value))
		}
	}

	impl Deserialize for MyStruct {
		fn deserialize(input: &str) -> Result<Self> {
			let parts: Vec<&str> = input.split(':').collect();
			if parts.len() != 2 {
				return Err(Error::DeserializeError(
					"Input string is not properly formatted".to_string(),
				));
			}
			let name = parts[0].to_string();
			let value = parts[1].parse::<i32>().map_err(|_| {
				Error::DeserializeError(
					"Value is not a valid integer".to_string(),
				)
			})?;
			Ok(MyStruct { name, value })
		}
	}

	#[test]
	fn test_serialize() {
		let my_obj = MyStruct {
			name:  "Test".to_string(),
			value: 42,
		};
		let serialized = my_obj.serialize().expect("Serialization failed");
		assert_eq!(serialized, "Test:42");
	}

	#[test]
	fn test_deserialize() {
		let serialized = "Test:42";
		let deserialized =
			MyStruct::deserialize(serialized).expect("Deserialization failed");
		assert_eq!(deserialized, MyStruct {
			name:  "Test".to_string(),
			value: 42,
		});
	}

	#[test]
	fn test_read_to_string() {
		let test_path = PathBuf::from("test_file.txt");
		write_to_file(&test_path, "Test Content")
			.expect("Failed to write test file");
		let content = read_to_string(&test_path).expect("Failed to read file");
		assert_eq!(content, "Test Content");
		fs::remove_file(test_path).expect("Failed to clean up test file");
	}

	#[test]
	fn test_encode_to_string() {
		let my_obj = MyStruct {
			name:  "Test".to_string(),
			value: 42,
		};
		let encoded = encode_to_string(&my_obj).expect("Encoding failed");
		assert_eq!(encoded, "Test:42");
	}

	#[test]
	fn test_decode_from_file() {
		let test_path = PathBuf::from("decode_test_file.txt");
		write_to_file(&test_path, "Test:42")
			.expect("Failed to write test file");

		let decoded: MyStruct =
			decode_from_file(test_path.clone()).expect("Decoding failed");
		assert_eq!(decoded, MyStruct {
			name:  "Test".to_string(),
			value: 42,
		});

		fs::remove_file(test_path).expect("Failed to clean up test file");
	}
}
