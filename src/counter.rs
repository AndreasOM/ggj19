use crate::bobs::bobtype::BobType;
use crate::fb::FB;

pub struct Counter {

}

impl Counter {
	fn offset_for( n: usize ) -> isize {
		match n {
			0 => 0,
			1 => 17,
			2 => 34,
			3 => 51,
			4 => 68,
			5 => 85,
			6 => 102,
			7 => 119,
			8 => 136,
			9 => 153,
			_ => 0,
		}
	}
	pub fn draw( value: usize, data: &Vec< u32 >, x: isize, y: isize, fb: &mut FB ) {
		let mut v = value;
		let mut x = x;
		const FONT_WIDTH: isize = 16;
		let width = 16;

//		while v > 0 {
		while {
			let n = v % 10;
			v = v / 10;
			let offset = Counter::offset_for( n ) * FONT_WIDTH;

			let num_height = 14;
			fb.blit_rect( x, y, x+FONT_WIDTH, y+num_height, data, offset, width );
//			fb.blit_rect( x, y, x+FONT_WIDTH, y+178, data, offset, width );

			x -= FONT_WIDTH-4;

			v > 0
		} {};
//		};
	}
}
