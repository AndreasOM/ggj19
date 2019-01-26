
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

	pub fn fill_rect( &mut self, sx: usize, sy: usize, ex: usize, ey: usize, col: u32 ) {
		let ex = if ex < self.width { ex } else { self.width };
		let ey = if ey < self.height { ey } else { self.height };


		for y in sy..ey {
			for x in sx..ex {
				self.buffer[ y * self.width + x ] = col;				
			}
		}
	}
}

