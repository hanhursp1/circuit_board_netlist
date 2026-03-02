use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub type Coordinates = (i32, i32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Netlist {
	// A set of nets, a pairing of the name and coordinate set
	pub nets: Vec<(Option<String>, Vec<Coordinates>)>,
}

impl Netlist {
	pub fn new(nets: Vec<(Option<String>, Vec<Coordinates>)>) -> Self {
		Self { nets }
	}

	pub fn num_vertices(&self) -> usize {
		self.nets.iter().fold(0, |n, e| n + e.1.len())
	}

	pub fn num_nets(&self) -> usize {
		self.nets.len()
	}

	// Creates a clone of `self` starting from `from` and ending with the max number of allowed vertices
	pub fn cloned_slice(&self, from: usize, with_vertices: usize) -> (usize, Self) {
		let n = &self.nets[from..];
		let (mut num_verts, mut num_nets) = (0, 0);
		for net in n.iter() {
			num_verts += net.1.len();
			if num_verts >= with_vertices {
				break;
			}
			num_nets += 1;
		}
		let n = if num_nets > 0 { &n[..num_nets] } else { &[] };
		(num_nets, Self::new(n.to_vec()))
	}

	pub fn sort_nets(&mut self) {
		// Sort each netlist vertex from left to right, top to bottom
		self
			.nets
			.iter_mut()
			.for_each(|(_, verts)| verts.sort_by(|(x0, y0), (x1, y1)| x0.cmp(x1).then(y0.cmp(y1))));
	}

	pub fn sort_shortest(&mut self) {
		self.sort_nets(); // First, sort the nets
		// Then, find a short path through all points
		// Use sort_by_cached_key because each vert group uses the center function, which is itself O(n)
		// If we didn't, and called center for each comparison, this sort would be O(n^2 log(n)),
		// With cached keys, it's O(n + n log(n)) = O(n log(n))
		self.nets.sort_by_cached_key(|(_, verts)| {
			// Find the "score" of each vertex grouping
			// We want to do vertex groups in the following order:
			//	1. From left to right
			//	2. From top to bottom
			//	3. From smallest to largest (i.e. do any nets that take up the whole board last)
			// For now, just sort by:
			//	1. Square magnitude of center
			let center = center(verts);
			let center = (center.0 as i64, center.1 as i64);
			let dist = (center.0 * center.0) as u64 + (center.1 * center.1) as u64;
			// let dist = (dist as f32).sqrt();	// sort_by_cached_key requires an Ord, so floats don't work
			dist
		});
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

fn center(verts: &[Coordinates]) -> Coordinates {
	let sum = verts.iter().fold((0i64, 0i64), |cum, i| {
		(cum.0 + i.0 as i64, cum.1 + i.1 as i64)
	});
	(
		(sum.0 / verts.len() as i64) as i32,
		(sum.1 / verts.len() as i64) as i32,
	)
}

#[derive(Debug, Default)]
struct BoundingBox {
	x: i32,
	y: i32,
	w: u32,
	h: u32,
}

impl BoundingBox {
	fn around(verts: &[Coordinates]) -> Self {
		let (mut x_min, mut x_max) = (i32::MAX, i32::MIN);
		let (mut y_min, mut y_max) = (i32::MAX, i32::MIN);

		for (x, y) in verts.iter() {
			if *x < x_min {
				x_min = *x
			};
			if *y < y_min {
				y_min = *y
			};
			if *x > x_max {
				x_max = *x
			};
			if *y > y_max {
				y_max = *y
			};
		}

		Self {
			x: x_min,
			y: y_min,
			w: x_min.abs_diff(x_max),
			h: y_min.abs_diff(y_max),
		}
	}

	fn center(&self) -> Coordinates {
		(self.x + (self.w / 2) as i32, self.y + (self.h / 2) as i32)
	}

	fn size(&self) -> u64 {
		(self.w * self.h) as u64
	}
}
