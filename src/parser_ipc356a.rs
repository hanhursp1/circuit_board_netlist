// TODO: Remove all panics, implement a custom error type

use std::{
	collections::HashMap
};

use crate::netlist::{Coordinates, Netlist};

// type Coordinates = (i32, i32);
type SignalName = [u8; 14];
type SignalNameSlice<'a> = &'a [u8];

#[derive(Debug, Clone)]
pub struct IPC356aNetlist {
	pub units: Units,
	pub nets: HashMap<SignalName, Vec<Coordinates>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Units {
	SI,     // Metric (millimeters and radians)
	InDeg,  // Inches and degrees
	MilDeg, // Millimeters and degrees
	InRad,  // Inches and radians
}

fn inches_to_um(coords: Coordinates) -> Coordinates {
	(
		((coords.0 as f32) * 2.54) as i32,
		((coords.1 as f32) * 2.54) as i32,
	)
}

fn sig_slice_to_fixed(slice: SignalNameSlice) -> SignalName {
	assert_eq!(slice.len(), 14);
	slice[0..14].try_into().unwrap()
}

impl IPC356aNetlist {
	pub fn new() -> Self {
		Self::with_units(Units::SI)
	}

	pub fn with_units(units: Units) -> Self {
		Self {
			units,
			nets: HashMap::new(),
		}
	}

	pub fn add_coord_set(&mut self, sig_name: SignalNameSlice, coords: Coordinates) {
		let sig_name: SignalName = sig_slice_to_fixed(sig_name);
		match self.nets.get_mut(&sig_name) {
			Some(v) => {
				v.push(coords);
			}
			None => {
				self.nets.insert(sig_name, vec![coords]);
			}
		}
	}

	pub fn has_coord_set(&self, sig_name: SignalNameSlice) -> bool {
		let sig_name = sig_slice_to_fixed(sig_name);
		self.nets.contains_key(&sig_name)
	}

	pub fn to_si_units(&mut self) {
		match self.units {
			Units::InDeg | Units::InRad => {
				for v in self.nets.values_mut() {
					for v in v.as_mut_slice() {
						*v = inches_to_um(*v);
					}
				}
				self.units = Units::SI;
			}
			_ => {}
		}
	}

	pub fn si_units(mut self) -> Self {
		self.to_si_units();
		self
	}
}

impl Into<Netlist> for IPC356aNetlist {
	fn into(self) -> Netlist {
		// Do this functionally
		Netlist::new(
			self
				// Convert to SI units, taking ownership
				.si_units()
				.nets
				// Make it an iterator, taking ownership of the contents
				.into_iter()
				// Equivalent to a for loop, but we can do it in-place and get an iterator
				.map(|(name, coords)| {
					(
						// Convert the ASCII array into a UTF-8 string, trim it, and take ownership
						Some(String::from_utf8_lossy(&name).trim_end().to_owned()),
						// We can just transfer ownership of the coords vec to the new struct
						coords,
					)
				})
				// Turn this iterator chain into a Vec
				.collect(),
		)
	}
}

fn parse_coord(coord: &[u8]) -> i32 {
	assert_eq!(coord.len(), 7);
	// Check whether or not the value is valid
	let is_neg = match coord[0] {
		b' ' | b'+' => false,
		b'-' => true,
		_ => panic!(
			"Coord slice \"{}\" is invalid",
			String::from_utf8_lossy(coord)
		),
	};
	let num = &coord[1..]; // The number itself
	// println!("{}", String::from_utf8_lossy(num));
	let mut pow = 100000; // 10^5
	let mut result: i32 = 0;
	for i in num {
		let digit = match i {
			b'0'..=b'9' => (i - b'0') as i32,
			b' ' => 0i32,
			_ => {
				panic!(
					"Coord slice \"{}\" is invalid",
					String::from_utf8_lossy(coord)
				)
			}
		};
		result += digit * pow;
		pow /= 10;
	}
	if is_neg { -result } else { result }
}

pub fn parse_netlist(s: &String) -> IPC356aNetlist {
	let mut netlist = IPC356aNetlist::new();

	for line in s.lines() {
		let line = line.as_bytes();
		match line[0..3] {
			// Through hole
			[b'3', b'1', b'7'] => {
				let name = &line[3..17];
				let x = parse_coord(&line[42..49]);
				let y = parse_coord(&line[50..57]);

				netlist.add_coord_set(name, (x, y));
			}
			// Header parameter
			[b'P', b' ', b' '] => {
				let mut line_iter = line[3..]
					.split(|&x| x == b' ')
					.filter(|&x| !x.is_empty())
					.clone();
				let kind = line_iter.next();
				if let Some(kind) = kind
					&& kind == b"UNITS"
				{
					netlist.units = match line_iter.next() {
						Some(b"SI") => Units::SI,
						Some(b"CUST0") | Some(b"CUST") => Units::InDeg,
						Some(b"CUST1") => Units::MilDeg,
						Some(b"CUST2") => Units::InRad,
						_ => panic!("Invalid netlist line: {}", String::from_utf8_lossy(line)),
					}
				}
			}
			// Default, unknown operation. Either continue or throw an error
			_ => {}
		}
	}
	netlist
}
