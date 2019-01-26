
#[derive(Debug)]
pub struct FB {
	pub width: usize,
	pub height: usize,
	buffer: Vec<u32>,
}
impl FB {
	pub fn new( width: usize, height: usize ) -> FB {

		FB {
			width: width,
			height: height,
			buffer: vec![0; width * height],
		}
	}

	pub fn buffer( &mut self ) -> &mut Vec<u32> {
		&mut self.buffer
	}
}

