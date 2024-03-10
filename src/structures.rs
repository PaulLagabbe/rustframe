/* --------------------------------------------------------------------------------------------- *
 * This file contains the definition of all the frame structures
 * Structure list:
 *	-	FrSH
 *	-	FrSE (deleted)
 *	-	FrameH
 *	-	FrAdcData
 *	-	FrDetector (TODO)
 *	-	FrEndOfFile
 *	-	FrEndOfFrame
 *	-	FrEvent (TODO)
 *	-	FrHistory (TODO)
 *	-	FrMsg (TODO)
 *	-	FrProcData
 *	-	FrRawData (TODO)
 *	-	FrSerData (TODO)
 *	-	FrSimData (TODO)
 *	-	FrSimEvent (TODO)
 *	-	FrSummary (TODO)
 *	-	FrTable (TODO)
 *	-	FrTOC
 *	-	FrVect
 * --------------------------------------------------------------------------------------------- */

pub mod bytes;

use std::io::{
	Bytes,
	BufReader,
};

use std::fs::File;

use crate::structures::bytes::*;

/* --------------------------------------------------------------------------------------------- *
 * define enum of struct
 * --------------------------------------------------------------------------------------------- */
#[derive(PartialEq, Debug)]
pub enum Structure {
	FrSH(FrSH),
	FrSE(FrSE),
	FrameH(FrameH),
	FrAdcData(FrAdcData),
	//FrDetector(FrDetector),
	FrEndOfFile(FrEndOfFile),
	FrEndOfFrame(FrEndOfFrame),
	//FrEvent(FrEvent),
	//FrHistory(FrHistory),
	//FrMsg(FrMsg),
	FrProcData(FrProcData),
	//FrRawData(FrRawData),
	//FrSerData(FrSerData),
	//FrSimData(FrSimData),
	//FrSimEvent(FrSimEvent),
	//FrSummary(FrSummary),
	//FrTable(FrTable),
	FrTOC(FrTOC),
	FrVect(FrVect),
}

impl Structure {

	// read common of each structure
	fn read_common(iterator: &mut Bytes<BufReader<File>>) -> (u8, u64, u32) {
		//println!("--------------------------------------------------");
		let length: u64 = read_u64(iterator);
		//println!("frame size: {}", length);
		let _chk_type: u8 = read_one_byte(iterator);
		let header_class: u8 = read_one_byte(iterator);
		//println!("structure type: {}", header_class);
		let instance: u32 = read_u32(iterator);
		//println!("n° instance: {}", instance);
		(header_class, length, instance)
	}

	// create a Structure object
	pub fn read(iterator: &mut Bytes<BufReader<File>>) -> Self {

		// read common part
		let (class, length, instance): (u8, u64, u32) = Self::read_common(iterator);

		// create a structure instance
		let output: Structure = match class {
			1 => Self::FrSH(FrSH::read(iterator, length, instance)),
			2 => Self::FrSE(FrSE::read(iterator, length, instance)),
			3 => Self::FrameH(FrameH::read(iterator, length, instance)),
			4 => Self::FrAdcData(FrAdcData::read(iterator, length, instance)),
			6 => Self::FrEndOfFile(FrEndOfFile::read(iterator, length, instance)),
			7 => Self::FrEndOfFrame(FrEndOfFrame::read(iterator, length, instance)),
			11 => Self::FrProcData(FrProcData::read(iterator, length, instance)),
			19 => Self::FrTOC(FrTOC::read(iterator, length, instance)),
			20 => Self::FrVect(FrVect::read(iterator, length, instance)),
			_ => panic!("Structure type not recognized: {}", class),
		};

		output
	}

	// skip 
	pub fn skip(iterator: &mut Bytes<BufReader<File>>) -> (u16, u32) {
		// read common part
		let (class, length, instance): (u8, u64, u32) = Self::read_common(iterator);
		let common_size: usize = 14;

		// skip the structure without reading it
		//iterator = iterator.skip(length as usize - common_size).clone();
		for _i in 0..(length as usize - common_size) { iterator.next(); }
		// return the class number, the number of bytes after the struct
		// the common part of the srtucture is 14 bytes long
		(class as u16, instance)
	}
	
	pub fn get_class(&self) -> u16 {
	
		match self {
			Self::FrSH(ref x) => x.class(),
			Self::FrSE(ref x) => x.class(),
			Self::FrameH(ref x) => x.class(),
			Self::FrAdcData(ref x) => x.class(),
			Self::FrEndOfFile(ref x) => x.class(),
			Self::FrEndOfFrame(ref x) => x.class(),
			Self::FrProcData(ref x) => x.class(),
			Self::FrTOC(ref x) => x.class(),
			Self::FrVect(ref x) => x.class(),
		}
	}


	pub fn get_instance(&self) -> u32 {
	
		match self {
			Self::FrSH(ref x) => x.get_instance(),
			Self::FrSE(ref x) => x.get_instance(),
			Self::FrameH(ref x) => x.get_instance(),
			Self::FrAdcData(ref x) => x.get_instance(),
			Self::FrEndOfFile(ref x) => x.get_instance(),
			Self::FrEndOfFrame(ref x) => x.get_instance(),
			Self::FrProcData(ref x) => x.get_instance(),
			Self::FrTOC(ref x) => x.get_instance(),
			Self::FrVect(ref x) => x.get_instance(),
		}
	}

	pub fn get_name(&self) -> String {
	
		match self {
			Self::FrSH(ref x) => x.name.clone(),
			Self::FrSE(ref x) => x.name.clone(),
			Self::FrameH(ref x) => x.name.clone(),
			Self::FrAdcData(ref x) => x.name.clone(),
			Self::FrEndOfFile(_) => panic!("No name for FrEndOfFile structure."),
			Self::FrEndOfFrame(_) => panic!("No name for FrEndOfFrame structure."),
			Self::FrProcData(ref x) => x.name.clone(),
			Self::FrTOC(_) => panic!("No name for FrTOC structure."),
			Self::FrVect(ref x) => x.name.clone(),
		}
	}
	
	pub fn get_start(&self) -> f64 {
		match self {
			Self::FrameH(ref x) => x.get_start(),
			_ => panic!("This structure does not have a gps start attribute"),
		}
	}
	pub fn get_end(&self) -> f64 {
		match self {
			Self::FrameH(ref x) => x.get_end(),
			_ => panic!("This structure does not have a dt attribute"),
		}
	}
	pub fn get_datavector_instance(&self) -> u32 {
		match self {
			Self::FrProcData(ref x) => x.get_datavector_instance(),
			Self::FrAdcData(ref x) => x.get_datavector_instance(),
			_ => panic!("This structure does not have a data vector."),
		}
	}
	pub fn get_compress(&self) -> u16 {
		match self {
			Self::FrVect(ref x) => x.get_compress(),
			_ => panic!("This structure does not have a compression level."),
		}
	}
	pub fn get_data(&self) -> Vec<u8> {
		match self {
			Self::FrVect(ref x) => x.get_data(),
			_ => panic!("This structure does not have a compression level."),
		}
	}
}

/* --------------------------------------------------------------------------------------------- *
 * define structures
 * --------------------------------------------------------------------------------------------- */
pub trait Reader {
	fn read(iterator: &mut Bytes<BufReader<File>>, length: u64, instance: u32) -> Self;
	fn class(&self) -> u16;
}

#[derive(PartialEq, Debug)]
pub struct FrSH {
	length: u64,
	instance: u32,
	name: String,
	class: u16,
	comment: String,
}

impl Reader for FrSH {
	
	fn read(iterator: &mut Bytes<BufReader<File>>,length: u64, instance: u32) -> Self {
			
		let name: String = read_one_string(iterator);
		//println!("name: '{}'", name);
		let class: u16 = read_u16(iterator);
		//println!("class number: {}", class);
		let comment: String = read_one_string(iterator);
		//println!("comment: '{}'", comment);
		let check_sum: u32 = read_u32(iterator);
		//println!("check sum: {}", check_sum);

		FrSH {
			length,
			instance,
			name,
			class,
			comment,
		}
	}

	fn class(&self) -> u16 {
		1
	}
}
// getter functions
impl FrSH {
	pub fn get_struct_class(&self) -> u16 {
		self.class
	}
	pub fn get_instance(&self) -> u32 {
		self.instance
	}
}
/* --------------------------------------------------------------------------------------------- */
#[derive(PartialEq, Debug)]
pub struct FrSE {
	length: u64,
	instance: u32,
	name: String,
	class: String,
	comment: String,
}
impl Reader for FrSE {
	fn read(iterator: &mut Bytes<BufReader<File>>,length: u64, instance: u32) -> Self {

		let name: String = read_one_string(iterator);
		//println!("name: '{}'", name);
		let class: String = read_one_string(iterator);
		//println!("class: '{}'", class);
		let comment: String = read_one_string(iterator);
		//println!("comment: '{}'", comment);
		let check_sum: u32 = read_u32(iterator);
		//println!("check sum: {}", check_sum);
		
		// create structure
		FrSE {
			length,
			instance,
			name,
			class,
			comment,
		}
	}
	fn class(&self) -> u16 {
		2
	}
}
impl FrSE {
	pub fn get_instance(&self) -> u32 {
		self.instance
	}
}

/* --------------------------------------------------------------------------------------------- */
#[derive(PartialEq, Debug)]
pub struct FrameH {
	length: u64,
	instance: u32,
	name: String,
	run: u32,
	frame: u32,
	gps_sec: u32,
	gps_nano: u32,
	dt: f64,
	data: Vec<(u16, u32)>,
}
impl Reader for FrameH {
	fn read(iterator: &mut Bytes<BufReader<File>>,length: u64, instance: u32) -> Self {

		let name: String = read_one_string(iterator);
		//println!("name: '{}'", name);
		let run = read_u32(iterator);
		//println!("run: {}", run);
		let frame = read_u32(iterator);
		//println!("frame: {}", frame);
		let data_quality = read_u32(iterator);
		//println!("quality: {}", data_quality);
		let gps_sec = read_u32(iterator);
		//println!("frame start: {} s", gps_sec);
		let gps_nano = read_u32(iterator);
		//println!("residual: {} ns", gps_nano);
		let u_leap: u16 = read_u16(iterator);
		//println!("utc to gpas leap: {}s", u_leap);
		let dt = read_f64(iterator);
		//println!("duration: {:.9} s", dt);

		// count the number of each structure instance:
		let mut class_count: Vec<(u16, u32)> = Vec::new();
		for _i in 0..13 {
			let (class, instance): (u16, u32) = read_ptr(iterator);
			//println!("class: {}, instance: {}", class, instance);
			if class != 0 {
				class_count.push((class, instance));
			}
		}
		let check_sum: u32 = read_u32(iterator);
		//println!("check sum: {}", check_sum);
		FrameH {
			length,
			instance,
			name,
			run,
			frame,
			gps_sec,
			gps_nano,
			dt,
			data: class_count,
		}
	}
	fn class(&self) -> u16 {
		3
	}
}

impl FrameH {
	
	pub fn get_start(&self) -> f64 {
		(self.gps_sec as f64) + (self.gps_nano as f64) * 1e-9
	}
	pub fn get_end(&self) -> f64 {
		self.dt + self.get_start()
	}
	pub fn get_instance(&self) -> u32 {
		self.instance
	}

}

/* --------------------------------------------------------------------------------------------- */
#[derive(PartialEq, Debug)]
pub struct FrAdcData {
	length: u64,
	instance: u32,
	name: String,
	comment: String,

	ch_group: u32,
	ch_num: u32,
	n_bits: u32,

	bias: f32,
	slope: f32,
	sample_rate: f64,
	time_offset: f64,
	f_shift: f64,
	phase: f32,

	valid_data: u32,
	data: (u16, u32),
	next: (u16, u32),
}

impl Reader for FrAdcData {
	fn read(iterator: &mut Bytes<BufReader<File>>,length: u64, instance: u32) -> Self {

		let name: String = read_one_string(iterator);
		//println!("name: '{}'", name);
		let comment: String = read_one_string(iterator);
		//println!("comment: '{}'", comment);
		let ch_group = read_u32(iterator);
		//println!("channel group: {}", ch_group);
		let ch_num = read_u32(iterator);
		//println!("channel number: {}", ch_num);
		let n_bits = read_u32(iterator);
		//println!("number of bits: {}", n_bits);
		let bias = read_f32(iterator);
		//println!("bias: {}", bias);
		let slope = read_f32(iterator);
		//println!("slope: {}", slope);
		let unit: String = read_one_string(iterator);
		//println!("unit: '{}'", unit);
		let sample_rate = read_f64(iterator);
		//println!("sample rate: {}", sample_rate);
		let time_offset = read_f64(iterator);
		//println!("time offset: {}", time_offset);
		let f_shift = read_f64(iterator);
		//println!("frequency shift: {}", f_shift);
		let phase = read_f32(iterator);
		//println!("phase: {}", phase);
		let valid_data = read_u32(iterator);
		//println!("data valid flag: {}", valid_data);
		// structure pointers
		let (class_data, data): (u16, u32) = read_ptr(iterator);
		//println!("class: {}, instance: {}", class_data, data);
		let (class_aux, aux): (u16, u32) = read_ptr(iterator);
		//println!("class: {}, instance: {}", class_aux, aux);
		let (class_next, next): (u16, u32) = read_ptr(iterator);
		//println!("class: {}, instance: {}", class_next, next);
		let check_sum: u32 = read_u32(iterator);
		//println!("check sum: {}", check_sum);
		
		// create structure
		FrAdcData {
			length,
			instance,
			name,
			comment,

			ch_group,
			ch_num,
			n_bits,

			bias,
			slope,
			sample_rate,
			time_offset,
			f_shift,
			phase,

			valid_data,
			data: (class_data, data),
			next: (class_next, next)
		}
	}
	fn class(&self) -> u16 {
		4
	}
}
// getter functions
impl FrAdcData {
	
	pub fn get_dump(&self) -> (f32, f32, f64, f64, f64, f32) {
		(self.bias, self.slope, self.sample_rate, self.time_offset, self.f_shift, self.phase)
	}

	pub fn get_datavector_instance(&self) -> u32 {
		self.data.1
	}
	pub fn get_instance(&self) -> u32 {
		self.instance
	}

}

/* --------------------------------------------------------------------------------------------- */
#[derive(PartialEq, Debug)]
pub struct FrEndOfFile {
	length: u64,
	instance: u32,
	n_frames: u32,
	n_bytes: u64,
	seek_toc: u64,
}

impl Reader for FrEndOfFile {
	fn read(iterator: &mut Bytes<BufReader<File>>,length: u64, instance: u32) -> Self {

		let n_frames = read_u32(iterator);
		//println!("# frames: {}", n_frames);
		let n_bytes = read_u64(iterator);
		//println!("total bytes: {}", n_bytes);
		let seek_toc = read_u64(iterator);
		//println!("# bytes to TOC: {} s", seek_toc);
		let check_sum_toc = read_u32(iterator);
		//println!("check sum TOC: {} ns", check_sum_toc);
		let check_sum_header = read_u32(iterator);
		//println!("check sum header: {} ns", check_sum_header);
		let check_sum: u32 = read_u32(iterator);
		//println!("check sum: {}", check_sum);
		let check_sum_file: u32 = read_u32(iterator);
		//println!("check sum file: {}", check_sum_file);

		FrEndOfFile {
			length,
			instance,
			n_frames,
			n_bytes,
			seek_toc,
		}

	}
	fn class(&self) -> u16 {
		6
	}
}
impl FrEndOfFile {
	pub fn get_instance(&self) -> u32 {
		self.instance
	}

}
/* --------------------------------------------------------------------------------------------- */
#[derive(PartialEq, Debug)]
pub struct FrEndOfFrame {
	length: u64,
	instance: u32,
	run: u32,
	frame: u32,
	gps_sec: u32,
	gps_nano: u32,
}

impl Reader for FrEndOfFrame {
	fn read(iterator: &mut Bytes<BufReader<File>>,length: u64, instance: u32) -> Self {
		
		let run = read_u32(iterator);
		//println!("run: {}", run);
		let frame = read_u32(iterator);
		//println!("frame: {}", frame);
		let gps_sec = read_u32(iterator);
		//println!("frame start: {} s", gps_sec);
		let gps_nano = read_u32(iterator);
		//println!("residual: {} ns", gps_nano);
		let check_sum: u32 = read_u32(iterator);
		//println!("check sum: {}", check_sum);

		FrEndOfFrame {
			length,
			instance,
			run,
			frame,
			gps_sec,
			gps_nano,
		}
	}
	fn class(&self) -> u16 {
		7
	}
}
impl FrEndOfFrame {
	pub fn get_instance(&self) -> u32 {
		self.instance
	}
}
/* --------------------------------------------------------------------------------------------- */
#[derive(PartialEq, Debug)]
pub struct FrProcData {
	length: u64,
	instance: u32,
	name: String,
	comment: String,

	object_type: u16,
	sub_type: u16,
	time_offset: f64,
	time_range: f64,
	f_shift: f64,
	phase: f32,
	f_range: f64,
	bw: f64,

	data: (u16, u32),
	next: (u16, u32)
}
impl Reader for FrProcData {
	fn read(iterator: &mut Bytes<BufReader<File>>,length: u64, instance: u32) -> Self {

		let name: String = read_one_string(iterator);
		//println!("name: '{}'", name);
		let comment: String = read_one_string(iterator);
		//println!("comment: '{}'", comment);
		let object_type: u16 = read_u16(iterator);
		//println!("type 1: {}", object_type);
		let sub_type: u16 = read_u16(iterator);
		//println!("type 2: {}", sub_type);
		let time_offset = read_f64(iterator);
		//println!("start time: {} s", time_offset);
		let time_range = read_f64(iterator);
		//println!("duration: {} s", time_range);
		let f_shift = read_f64(iterator);
		//println!("frequency shift: {} Hz", f_shift);
		let phase = read_f32(iterator);
		//println!("phase: {} rad", phase);
		let f_range = read_f64(iterator);
		//println!("frequency range: {} Hz", f_range);
		let bw = read_f64(iterator);
		//println!("bandwidth: {} Hz", bw);
		
		// auxiliary parameters
		let n_aux: u16 = read_u16(iterator);
		//println!("# of auxiliary parameters: {}", n_aux);
		let mut param: Vec<f64> = Vec::new();
		let mut param_name: Vec<String> = Vec::new();
		for _i in 0..n_aux {
			param.push(read_f64(iterator));
		}
		for _i in 0..n_aux {
			param_name.push(read_one_string(iterator));
		}
		//println!("names: {:#?}", param_name);
		//println!("values: {:#?}", param);	
		// structure pointers
		let (class_data, data): (u16, u32) = read_ptr(iterator);
		//println!("class: {}, instance: {}", class_data, data);
		let (class_aux, aux): (u16, u32) = read_ptr(iterator);
		//println!("class: {}, instance: {}", class_aux, aux);
		let (class_table, table): (u16, u32) = read_ptr(iterator);
		//println!("class: {}, instance: {}", class_table, table);
		let (class_history, history): (u16, u32) = read_ptr(iterator);
		//println!("class: {}, instance: {}", class_history, history);
		let (class_next, next): (u16, u32) = read_ptr(iterator);
		//println!("class: {}, instance: {}", class_next, next);

		let check_sum: u32 = read_u32(iterator);
		//println!("check sum: {}", check_sum);
		// create structure
		FrProcData {
			length,
			instance,
			name,
			comment,

			object_type,
			sub_type,
			time_offset,
			time_range,
			f_shift,
			phase,
			f_range,
			bw,

			data: (class_data, data),
			next: (class_next, next)
		}
	}
	fn class(&self) -> u16 {
		11
	}
}
// getter functions
impl FrProcData {
	
	pub fn get_dump(&self) -> (u16, u16, f64, f64, f64, f32, f64, f64) {
		(self.object_type, self.sub_type, self.time_offset, self.time_range, self.f_shift,
		self.phase, self.f_range, self.bw)
	}

	pub fn get_datavector_instance(&self) -> u32 {
		self.data.1
	}

	pub fn get_instance(&self) -> u32 {
		self.instance
	}

}
/* --------------------------------------------------------------------------------------------- */
/*
pub struct FrTable {
	length: u64,
	instance: u32,
}
impl Reader for FrTable {
	fn read(iterator: &mut Bytes<BufReader<File>>,length: u64, instance: u32) -> Self {

	}
}
*/
/* --------------------------------------------------------------------------------------------- */
// TODO: add the structure types in the table of contents
#[derive(PartialEq, Debug)]
pub struct FrTOC {
	length: u64,
	instance: u32,
	// frames
	leap: u16,
	gps_sec: Vec<u32>,
	gps_nano: Vec<u32>,
	dt: Vec<f64>,
	position: Vec<u64>,
	// FrSH
	sh_id: Vec<u16>,
	sh_name: Vec<String>,
	// FrAdcData
	name_adc: Vec<String>,
	position_adc: Vec<Vec<u64>>,
	// FrProcData
	name_proc: Vec<String>,
	position_proc: Vec<Vec<u64>>,
}
impl Reader for FrTOC {
	fn read(iterator: &mut Bytes<BufReader<File>>,length: u64, instance: u32) -> Self {
		
		// frames
		let leap: u16 = read_u16(iterator);
		//println!("time leap: {}", leap);
		let mut gps_sec: Vec<u32> = Vec::new();
		let mut gps_nano: Vec<u32> = Vec::new();
		let mut dt: Vec<f64> = Vec::new();
		let mut position: Vec<u64> = Vec::new();
		// data parameters
		let n_frames: u32 = read_u32(iterator);
		//println!("# frames: {}", n_frames);
		for _i in 0..n_frames { read_u32(iterator); } // data quality
		for _i in 0..n_frames { gps_sec.push(read_u32(iterator)); }
		for _i in 0..n_frames { gps_nano.push(read_u32(iterator)); }
		for _i in 0..n_frames { dt.push(read_f64(iterator)); }
		for _i in 0..n_frames { read_u32(iterator); } // run
		for _i in 0..n_frames { read_u32(iterator); } // frame
		for _i in 0..n_frames { position.push(read_u64(iterator)); }
		// data position in bytes
		for _i in 0..n_frames { read_u64(iterator); } // adc
		for _i in 0..n_frames { read_u64(iterator); } // serial
		for _i in 0..n_frames { read_u64(iterator); } // table
		for _i in 0..n_frames { read_u64(iterator); } // message
		// FrSH
		let mut sh_id: Vec<u16> = Vec::new();
		let sh_name: Vec<String> = Vec::new();
		let n_sh: u16 = read_u16(iterator);
		//println!("# headers: {}", n_sh);
		for _i in 0..n_sh { sh_id.push(read_u16(iterator)); }
		//println!("{:#?}", sh_id);
		for _i in 0..n_sh { read_one_string(iterator); }
		//println!("{:#?}", sh_name);
		// FrDetector
		let n_detector: u32 = read_u32(iterator);
		//println!("# detectors: {}", n_detector);
		for _i in 0..n_detector { read_one_string(iterator); }
		for _i in 0..n_detector { read_u64(iterator); }
		// FrStatData
		let mut n_static: u32 = read_u32(iterator);
		//println!("# static type: {}", n_static);
		for _i in 0..n_static { read_one_string(iterator); }
		for _i in 0..n_static { read_one_string(iterator); }
		for _i in 0..n_static { read_u32(iterator); }
		n_static = read_u32(iterator);
		for _i in 0..n_static { read_u32(iterator); }
		for _i in 0..n_static { read_u32(iterator); }
		for _i in 0..n_static { read_u32(iterator); }
		for _i in 0..n_static { read_u64(iterator); }
		// FrAdcData
		let mut name_adc: Vec<String> = Vec::new();
		let mut position_adc: Vec<Vec<u64>> = Vec::new();
		let n_adc: u32 = read_u32(iterator);
		//println!("# adc: {}", n_adc);
		for _i in 0..n_adc { name_adc.push(read_one_string(iterator)); }
		for _i in 0..n_adc { read_u32(iterator); } // channel id
		for _i in 0..n_adc { read_u32(iterator); } // group id
		for _i in 0..n_adc {
			let mut one_vect: Vec<u64> = Vec::new();
			for _j in 0..n_frames { one_vect.push(read_u64(iterator)); }
			position_adc.push(one_vect);
		}
		// FrProcData
		let mut name_proc: Vec<String> = Vec::new();
		let mut position_proc: Vec<Vec<u64>> = Vec::new();
		let n_proc: u32 = read_u32(iterator);
		//println!("# processed: {}", n_proc);
		for _i in 0..n_proc { name_proc.push(read_one_string(iterator)); }
		for _i in 0..n_proc {
			let mut one_vect: Vec<u64> = Vec::new();
			for _j in 0..n_frames { one_vect.push(read_u64(iterator)); }
			position_proc.push(one_vect);
		}
		// FrSimData
		let n_sim: u32 = read_u32(iterator);
		//println!("# simulations: {}", n_sim);
		for _i in 0..n_sim { read_one_string(iterator); }
		for _i in 0..n_sim {
			for _j in 0..n_frames { read_u64(iterator); }
		}
		// FrSerData
		let n_ser: u32 = read_u32(iterator);
		//println!("# serial: {}", n_ser);
		for _i in 0..n_ser { read_one_string(iterator); }
		for _i in 0..n_ser {
			for _j in 0..n_frames { read_u64(iterator); }
		}
		// FrSummary
		let n_summary: u32 = read_u32(iterator);
		//println!("# summary: {}", n_summary);
		for _i in 0..n_summary { read_one_string(iterator); }
		for _i in 0..n_summary {
			for _j in 0..n_frames { read_u64(iterator); }
		}
		// FrEvent
		let n_event_type: u32 = read_u32(iterator);
		//println!("# event type: {}", n_event_type);
		for _i in 0..n_event_type { read_one_string(iterator); }
		for _i in 0..n_event_type { read_u32(iterator); }

		let n_event: u32 = read_u32(iterator);
		//println!("# event: {}", n_event);
		for _i in 0..n_event { read_u32(iterator); }
		for _i in 0..n_event { read_u32(iterator); }
		for _i in 0..n_event { read_f32(iterator); }
		for _i in 0..n_event { read_u64(iterator); }
		// FrSimEvent
		let n_event_type: u32 = read_u32(iterator);
		//println!("# simulated event type: {}", n_event);
		for _i in 0..n_event_type { read_one_string(iterator); }
		for _i in 0..n_event_type { read_u32(iterator); }

		let n_event: u32 = read_u32(iterator);
		//println!("# simulated event: {}", n_event);
		for _i in 0..n_event { read_u32(iterator); }
		for _i in 0..n_event { read_u32(iterator); }
		for _i in 0..n_event { read_f32(iterator); }
		for _i in 0..n_event { read_u64(iterator); }
		
		let check_sum: u32 = read_u32(iterator);
		//println!("check sum: {}", check_sum);

		// create struct
		FrTOC {
			length,
			instance,
			// frames
			leap,
			gps_sec,
			gps_nano,
			dt,
			position,
			// FrSH
			sh_id,
			sh_name,
			// FrAdcData
			name_adc,
			position_adc,
			// FrProcData
			name_proc,
			position_proc,

		}
	}
	fn class(&self) -> u16 {
		19
	}
}
impl FrTOC {
	pub fn get_instance(&self) -> u32 {
		self.instance
	}

}
/* --------------------------------------------------------------------------------------------- */
#[derive(PartialEq, Debug)]
pub struct FrVect {
	length: u64,
	instance: u32,
	name: String,
	compress: u16,

	data_type: u16,
	n_data: u64,
	n_bytes: u64,
	data: Vec<u8>,

	n_dim: u32,
	dim: Vec<u64>,
	dx: Vec<f64>,
	start_x: Vec<f64>,
	unit_x: Vec<String>,
	unit_y: String,
	
	next: (u16, u32),
}
impl Reader for FrVect {
	fn read(iterator: &mut Bytes<BufReader<File>>,length: u64, instance: u32) -> Self {

		let name: String = read_one_string(iterator);
		//println!("name: '{}'", name);
		let compress = read_u16(iterator);
		//println!("compression: {}", compress);
		let data_type = read_u16(iterator);
		//println!("data type {}", data_type);
		let n_data = read_u64(iterator);
		//println!("# data: {}", n_data);
		// read data
		let n_bytes = read_u64(iterator);
		//println!("# bytes: {}", n_bytes);
		let mut data: Vec<u8> = Vec::new();
		for _i in 0..n_bytes { data.push(read_one_byte(iterator)); }
		// dimension
		let n_dim = read_u32(iterator);
		//println!("# dim: {}", n_dim);
		let mut dim: Vec<u64> = Vec::new();
		let mut dx: Vec<f64> = Vec::new();
		let mut start_x: Vec<f64> = Vec::new();
		let mut unit_x: Vec<String> = Vec::new();
		for _i in 0..n_dim { dim.push(read_u64(iterator)); }
		for _i in 0..n_dim { dx.push(read_f64(iterator)); }
		for _i in 0..n_dim { start_x.push(read_f64(iterator)); }
		for _i in 0..n_dim { unit_x.push(read_one_string(iterator)); }
		//println!("unit {:#?}", unit_x);
		let unit_y: String = read_one_string(iterator);
		//println!("unit: '{}'", unit_y);
		let (class_next, next): (u16, u32) = read_ptr(iterator);
		//println!("class: {}, instance: {}", class_next, next);
		let check_sum: u32 = read_u32(iterator);
		//println!("check sum: {}", check_sum);
		
		// create structure
		FrVect {
			length,
			instance,
			name,
			compress,

			data_type,
			n_data,
			n_bytes,
			data,

			n_dim,
			dim,
			dx,
			start_x,
			unit_x,
			unit_y,
			
			next: (class_next, next),
		}
	}
	fn class(&self) -> u16 {
		20
	}
}
impl FrVect {
	pub fn get_instance(&self) -> u32 {
		self.instance
	}
	pub fn get_compress(&self) -> u16 {
		self.compress
	}
	pub fn get_data(&self) -> Vec<u8> {
		self.data.clone()
	}
}

