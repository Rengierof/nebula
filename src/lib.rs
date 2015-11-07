#![crate_name = "nebula"]

#![crate_type = "rlib"]
#![crate_type = "dylib"]

#[macro_use]
extern crate glium;

pub use self::window::Display;

mod window;

#[derive(Debug)]
pub enum Color {
	White,
	Black,
}

impl Color {
	pub fn get_color_values(&self) -> [f32; 3] {
		match *self {
			Color::White => [0f32, 0f32, 0f32],
			Color::Black => [255f32, 255f32, 255f32],
		}
	}
}

/// TODO: Possibly implement alpha to vertices

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
	pub position: [f32; 2],
	pub color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

impl Vertex {
	pub fn new(pos: [f32; 2], col: [f32; 3]) -> Vertex {
		Vertex { position: pos, color: col }
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_color() {
		use super::*;
		
		let col1 = Color::White;
		let arr1 = col1.get_color_values();
		println!("{:?}", col1.get_color_values());
		
		let col2 = Color::Black;
		let arr2 = col2.get_color_values();
		println!("{:?}", col2.get_color_values());
		
		assert_eq!(arr1[0], 0f32);
		assert_eq!(arr2[0], 255f32);
	}
}
