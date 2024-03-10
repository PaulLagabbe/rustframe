/* --------------------------------------------------------------------------------------------- *
 * Libraries
 * --------------------------------------------------------------------------------------------- */

pub mod structures;

use std::fs::File;
use std::io::{
    Bytes,
    BufReader,
};
use crate::structures::{
	*,
	bytes::*,
};

/* --------------------------------------------------------------------------------------------- *
 * Define frame struct
 * --------------------------------------------------------------------------------------------- */



/* --------------------------------------------------------------------------------------------- *
 * Frame reader functions
 * --------------------------------------------------------------------------------------------- */
pub fn read_file_header(iterator: &mut Bytes<BufReader<File>>) {
	
	// read first string
	// make a vector of bytes
	let mut binary: Vec<u8> = Vec::new();
	for _i in 0..5 {
		binary.push(iterator.next().unwrap().unwrap());
	}
	// cast it into a string
	let s = match std::str::from_utf8(&binary) {
		Ok(v) => v,
		Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
	};
	println!("{}", s.to_string());
	let data_format: u8 = read_one_byte(iterator);
	println!("data format: {}", data_format);
	// library
	let library: u8 = read_one_byte(iterator);
	println!("frame library version: {}", library);
	// size of different types
	println!("size of types");
	let _size_u16: u8 = read_one_byte(iterator);
	let _size_u32: u8 = read_one_byte(iterator);
	let _size_u64: u8 = read_one_byte(iterator);
	let _size_f32: u8 = read_one_byte(iterator);
	let _size_f64: u8 = read_one_byte(iterator);
	// read test number
	let _num1 = read_u16(iterator);
	let _num2 = read_u32(iterator);
	let _num3 = read_u64(iterator);
	println!("pi:");
	let pi_f32 = read_f32(iterator);
	println!("{}", pi_f32);
	assert_eq!(pi_f32, std::f32::consts::PI);
	let pi_f64 = read_f64(iterator);
	println!("{}", pi_f64);
	assert_eq!(pi_f64, std::f64::consts::PI);
	// 
	let frame_lib: u8 = iterator.next().unwrap().unwrap();
	print!("frame library:");
	match frame_lib {
		0 => println!("unknown"),
		1 => println!("frameL"),
		2 => println!("frameCCP"),
		_ => println!("unused"),
	}
	let checksum: u8 = iterator.next().unwrap().unwrap();
	print!("frame library:");
	match checksum {
		0 => println!("non"),
		1 => println!("CRC"),
		_ => println!("unused"),
	}
	println!("--------------------------------------------------");
}

/* --------------------------------------------------------------------------------------------- */
pub fn read_frame_header(iterator: &mut Bytes<BufReader<File>>) -> (Structure, u16) {
	
	// read first structure and verify that it is a FrSH instance
	let frsh = Structure::read(iterator);

	// get structure class
	let struct_class: u16 = match frsh {
		Structure::FrSH(ref x) => x.get_struct_class(),
		_ => panic!("I am not supposed to panic there. Here is my structure type: {:#?}",
			frsh),
	};
	// return the first structture and the class number of the frame
	(frsh, struct_class)
}

/* --------------------------------------------------------------------------------------------- */
pub fn read_frame(iterator: &mut Bytes<BufReader<File>>, frsh: Structure,
	name: String, gps_start: f64, gps_end: f64) -> (Vec<Structure>, Vec<Structure>) {

	// read FrameH structure

	// verify that the structure is a FrameH instance
	let struct_class = match frsh {
		Structure::FrSH(ref x) => x.get_struct_class(),
		_ => panic!("{:#?}", frsh),
	};
	assert_eq!(struct_class, 3); 
	
	// initialize dictionary vector
	let mut struct_list: Vec<Structure> = Vec::new();
	let mut vect_list: Vec<Structure> = Vec::new();
	let mut instance_list: Vec<u32> = Vec::new();

	// read FrameH structure, and check the gps time
	let mut current: Structure = read_one_structure(iterator, frsh);

	// read structure header
	let (mut frsh, mut class): (Structure, u16) = read_frame_header(iterator);

		
	let is_time_matching: bool = 
		!((current.get_start() > gps_end) & (current.get_end() > gps_end)
		| (current.get_start() < gps_start) & (current.get_end() < gps_start));
	// check if the frame time match with the given time


	while class != 7 {
		
		// check structure class
		match class {
			// FrAdcData or FrProcData, and add
			4 | 11 => {
				current = read_one_structure(iterator, frsh);
				// add struct to the list if the names and gps times correspond to the given ones
				if (current.get_name() == name) & is_time_matching {
					instance_list.push(current.get_datavector_instance());
					struct_list.push(current);
				}
				(frsh, class) = read_frame_header(iterator);
			},
			// read FrVect
			20 => {
				current = read_one_structure(iterator, frsh);
				// add FrVect in the list if the instances contains its instance
				if instance_list.contains(&current.get_instance()) {
					vect_list.push(current);
				}
				(frsh, class) = read_frame_header(iterator);
			},
			// error message
			1 | 2 | 3 | 6 | 7 | 19 => {
				println!("The structure {:#?} is not suppose to come here.", frsh);
			},
			// Structures not recognized
			_ => {
				println!("The structure {:#?} is not recognized yet.", frsh);
			},
		};

	}
	// read end of frame structure (unused)
	let _ = read_one_structure(iterator, frsh);

	(struct_list, vect_list)
}



/* --------------------------------------------------------------------------------------------- */
pub fn read_one_structure(iterator: &mut Bytes<BufReader<File>>, frsh: Structure) -> Structure {
	
	// read dictionary
	// read structure header and verify if it is a FrTOC
	let mut dictionary = Vec::new();
	let struct_class = match frsh {
		Structure::FrSH(ref x) => x.get_struct_class(),
		_ => panic!("wrong structure {:#?}", frsh),
	};
	dictionary.push(frsh);
	// read structure elements
	let mut current = Structure::read(iterator);
	while current.get_class() == 2 {
		dictionary.push(current);
		current = Structure::read(iterator);
	}
	// check the structure class
	assert_eq!(current.get_class(), struct_class);

	// return FrTOC structure
	current
}

/* --------------------------------------------------------------------------------------------- */
pub fn skip_one_structure(iterator: &mut Bytes<BufReader<File>>, frsh: Structure) -> u32 {
	
	// read dictionary
	let struct_class = match frsh {
		Structure::FrSH(ref x) => x.get_struct_class(),
		_ => panic!("wrong structure {:#?}", frsh),
	};	

	// read structure elements
	let (mut class, mut instance): (u16, u32) = Structure::skip(iterator);
	while class == 2 {
		(class, instance) = Structure::skip(iterator);

	}
	assert_eq!(class, struct_class);
	// return FrTOC structure
	instance
}







