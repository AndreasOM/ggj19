
#[derive(Debug)]
pub struct FB {
	pub width: isize,
	pub height: isize,
	buffer: Vec<u32>,
}
impl FB {
	pub fn new( width: isize, height: isize ) -> FB {

		FB {
			width: width,
			height: height,
			buffer: vec![0; ( width * height ) as usize],
		}
	}

	pub fn buffer( &mut self ) -> &mut Vec<u32> {
		&mut self.buffer
	}

	pub fn mix( l: u32, r: u32, m: f32 ) -> u32 {
		let im = 1.0 - m;
		let lr = ( ( ( l >> 24 ) & 0xff ) as f32 * m ) as u32;
		let lg = ( ( ( l >> 16 ) & 0xff ) as f32 * m ) as u32;
		let lb = ( ( ( l >>  8 ) & 0xff ) as f32 * m ) as u32;
		let la = ( ( ( l >>  0 ) & 0xff ) as f32 * m ) as u32;
		let rr = ( ( ( r >> 24 ) & 0xff ) as f32 * im ) as u32;
		let rg = ( ( ( r >> 16 ) & 0xff ) as f32 * im ) as u32;
		let rb = ( ( ( r >>  8 ) & 0xff ) as f32 * im ) as u32;
		let ra = ( ( ( r >>  0 ) & 0xff ) as f32 * im ) as u32;

		( ( lr + rr ) << 24 )
		| ( ( lg + rg ) << 16 )
		| ( ( lb + rb ) <<  8 )
		| ( ( la + ra ) <<  0 )

	}

	pub fn fill_rect( &mut self, sx: isize, sy: isize, ex: isize, ey: isize, col: u32 ) {
		let ex = if ex < self.width { ex } else { self.width };
		let ey = if ey < self.height { ey } else { self.height };


		for y in sy..ey {
			for x in sx..ex {
				let p = ( y * self.width + x ) as usize;
				self.buffer[ p ] = col;				
			}
		}
	}
}

