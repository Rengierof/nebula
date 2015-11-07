use glium;
use glium::glutin;

use super::Vertex;

/// Display works as the surface for rendering.
/// TODO: Implement `Debug` manually, because the backend doesn't derive it.
pub struct Display {
	/// The backend of the window from glium
	backend: glium::backend::glutin_backend::GlutinFacade,
	
	/// The size on the x-axis.
	frame_size_x: i32,
	
	/// The size on the y-axis.
	frame_size_y: i32,

	/// The shader program
	program: glium::Program,

	/// The vertex buffer
	vertex_buffer: glium::VertexBuffer<Vertex>,
}

impl Display {
	pub fn new() -> Option<Display> {
		use glium::{ DisplayBuild, Surface };
	
		let glium_display = glutin::WindowBuilder::new()
									.build_glium()
									.unwrap();
	
	}
	
	/// Returns the size of the display.
	/// TODO: Size defined in the structure should change as the size of the window changes.
	pub fn get_size(&self) -> (i32, i32) {
		(self.frame_size_x, self.frame_size_y)
	}
}