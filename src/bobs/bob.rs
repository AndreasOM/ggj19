use image::GenericImageView;

#[derive(Debug)]
pub struct Bob {
	pub width: isize,
	pub height: isize,
	pub data: Vec< u32 >,
}

impl Bob {
	pub fn new() -> Bob {
		Bob {
			width: 0,
			height: 0,
			data: Vec::new(),
		}
	}

	pub fn load( &mut self, filename: &str ) {
		let img = image::open(filename).unwrap();
	}

	pub fn load_png_bytes( &mut self, data: &[u8] ) {
		let img = image::load_from_memory_with_format( data, image::ImageFormat::PNG ).unwrap();

		self.width = img.dimensions().0 as isize;
		self.height = img.dimensions().1 as isize;

		img.to_rgba();
		for p in img.pixels() {
			let d = p.2.data;
			self.data.push(
				// let mut trash_color =  ( 0xff << 24 ) | ( r << 16 ) | ( g << 8 ) | ( b << 0 );
				// ARGB = RGBA !
				// BGRA = RGBA
				  ( ( d[ 0 ] as u32 ) << 16 )	// R
				| ( ( d[ 1 ] as u32 ) <<  8 )	// G
				| ( ( d[ 2 ] as u32 ) <<  0 )	// B
				| ( ( d[ 3 ] as u32 ) << 24 )	// A
			);
		}
	}
}


// TQ is YEL
// BG RG