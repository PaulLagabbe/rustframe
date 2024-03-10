use std::str;
use std::fs::File;
use std::io::{
	Bytes,
	BufReader,
};



/* --------------------------------------------------------------------------------------------- *
 * read types
 * --------------------------------------------------------------------------------------------- */
pub fn read_one_string(iterator: &mut Bytes<BufReader<File>>) -> String {

	// read string length
	let length: u16 = read_u16(iterator);
	// make a vector of bytes
	let mut binary: Vec<u8> = Vec::new();
	for _i in 0..length {
		let result = match iterator.next() {
			Some(x) => x,
			None => panic!("The buffer reader return nothing."),
		};
		let value = match result {
			Ok(x) => x,
			Err(_) => panic!("The returned value is unreadable."),
		};
		binary.push(value);
	}
	// cast it into a string
	let s = match str::from_utf8(&binary) {
		Ok(v) => v,
		Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
	};
	s.to_string()
}

pub fn read_one_byte(iterator: &mut Bytes<BufReader<File>>) -> u8 {

	let one_byte: u8 = iterator.next().unwrap().unwrap();
	//println!("{}", one_byte);
	one_byte
}

/* --------------------------------------------------------------------------------------------- */
pub fn read_u16(iterator: &mut Bytes<BufReader<File>>) -> u16 {
	
	let mut binary: Vec<u8> = Vec::new();
	for _i in 0..2 {
		binary.push(iterator.next().unwrap().unwrap());
	}

	u16::from_le_bytes(binary.try_into().unwrap())
}
pub fn read_u32(iterator: &mut Bytes<BufReader<File>>) -> u32 {
	
	let mut binary: Vec<u8> = Vec::new();
	for _i in 0..4 {
		binary.push(iterator.next().unwrap().unwrap());
	}

	u32::from_le_bytes(binary.try_into().unwrap())
}
pub fn read_u64(iterator: &mut Bytes<BufReader<File>>) -> u64 {
	
	let mut binary: Vec<u8> = Vec::new();
	for _i in 0..8 {
		binary.push(iterator.next().unwrap().unwrap());
	}

	u64::from_le_bytes(binary.try_into().unwrap())
}
pub fn read_f32(iterator: &mut Bytes<BufReader<File>>) -> f32 {
	
	let mut binary: Vec<u8> = Vec::new();
	for _i in 0..4 {
		binary.push(iterator.next().unwrap().unwrap());
	}

	f32::from_le_bytes(binary.try_into().unwrap())
}
pub fn read_f64(iterator: &mut Bytes<BufReader<File>>) -> f64 {
	
	let mut binary: Vec<u8> = Vec::new();
	for _i in 0..8 {
		binary.push(iterator.next().unwrap().unwrap());
	}

	f64::from_le_bytes(binary.try_into().unwrap())
}
pub fn read_ptr(iterator: &mut Bytes<BufReader<File>>) -> (u16, u32) {

	let class: u16 = read_u16(iterator);
	let instance: u32 = read_u32(iterator);
	(class, instance)
}





