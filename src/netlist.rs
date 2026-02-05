use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub type Coordinates = (i32, i32);


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Netlist {
	// A set of nets, a pairing of the name and coordinate set
	pub nets: Vec<(Option<String>, Vec<Coordinates>)>
}

impl Netlist {
	pub fn new(nets: Vec<(Option<String>, Vec<Coordinates>)>) -> Self {
		Self {nets}
	}
}

impl Display for Netlist {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for (i, (s, c)) in self.nets.iter().enumerate() {
			f.write_fmt(format_args!("Net {}", i))?;
			if let Some(s) = s {
				f.write_fmt(format_args!(" ({})", s))?
			}
			f.write_fmt(format_args!(":\n\t{:?}\n", c))?
		}
		Ok(())
	}
}