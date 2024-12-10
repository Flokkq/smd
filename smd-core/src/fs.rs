use crate::error::Result;
use std::{
	fs,
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
	Ok(fs::read_to_string(path)?)
}

/// Serialize an object to a string.
pub fn encode_to_string<T>(value: &T) -> Result<String>
where
	T: Serialize,
{
	value.serialize()
}

/// Deserialize an object from a file.
pub fn decode_from_file<T>(path: PathBuf) -> Result<T>
where
	T: Deserialize,
{
	let content = read_to_string(&path)?;
	T::deserialize(&content)
}

#[cfg(test)]
mod tests {
	use crate::error::Error;

	use super::*;
	use std::fs;

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
		fs::write(&test_path, "Test Content")
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
		fs::write(&test_path, "Test:42").expect("Failed to write test file");

		let decoded: MyStruct =
			decode_from_file(test_path.clone()).expect("Decoding failed");
		assert_eq!(decoded, MyStruct {
			name:  "Test".to_string(),
			value: 42,
		});

		fs::remove_file(test_path).expect("Failed to clean up test file");
	}
}
