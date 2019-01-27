
use crate::bobs::bob::Bob;
use crate::bobs::bobtype::BobType;
use crate::fb::FB;

use std::collections::HashMap;

#[derive(Debug)]
pub struct BobManager {
	bobs: HashMap< BobType, Bob >,
}

impl BobManager {
	pub fn new() -> BobManager {
		BobManager {
			bobs: HashMap::new(),
		}
	}

	fn load( &mut self, bob_type: BobType, filename: &str ) {
		let mut bob = Bob::new();
		bob.load( filename );
		self.bobs.insert( bob_type, bob );
	}
	fn load_png_bytes( &mut self, bob_type: BobType, data: &[u8] ) {
		let mut bob = Bob::new();
		bob.load_png_bytes( &data );
		self.bobs.insert( bob_type, bob );
	}
	pub fn load_all( &mut self ) {
		{
			let bytes = include_bytes!("./../../resources/title.png");
			self.load_png_bytes( BobType::Title, bytes );
		}
		{
			let bytes = include_bytes!("./../../resources/gameover.png");
			self.load_png_bytes( BobType::GameOver, bytes );
		}
		{
			let bytes = include_bytes!("./../../resources/help.png");
			self.load_png_bytes( BobType::Help, bytes );
		}
		{
			let bytes = include_bytes!("./../../resources/numbers.png");
			self.load_png_bytes( BobType::Numbers, bytes );
		}
		{
			let bytes = include_bytes!("./../../resources/level_00.png");
			self.load_png_bytes( BobType::Background, bytes );
		}
		{
			let bytes = include_bytes!("./../../resources/target.png");
			self.load_png_bytes( BobType::Target, bytes );
		}
		{
			let bytes = include_bytes!("./../../resources/player_left.png");
			self.load_png_bytes( BobType::PlayerLeft, bytes );
		}
		{
			let bytes = include_bytes!("./../../resources/player_right.png");
			self.load_png_bytes( BobType::PlayerRight, bytes );
		}
		{
			let bytes = include_bytes!("./../../resources/player_up.png");
			self.load_png_bytes( BobType::PlayerUp, bytes );
		}
		{
			let bytes = include_bytes!("./../../resources/player_down.png");
			self.load_png_bytes( BobType::PlayerDown, bytes );
		}
		{
			let bytes = include_bytes!("./../../resources/trash_00.png");
			self.load_png_bytes( BobType::Trash00, bytes );
		}
	}

	// BGRA
	fn blend( fg: u32, bg: u32 ) -> u32 {
//		l
		let a = ( ( fg >> 24 ) & 0xff ) as f32 / 255.0;// + 1;
		FB::mix( fg, bg, a )
		/*
		let a = ( ( fg >> 0 ) & 0xff ) as u32;// + 1;
		let ia = 255 - a;
		let fg_r = ( fg >> 8 ) & 0xff;
		let fg_g = ( fg >> 16 ) & 0xff;
		let fg_b = ( fg >> 24 ) & 0xff;
		let bg_r = ( bg >> 8 ) & 0xff;
		let bg_g = ( bg >> 16 ) & 0xff;
		let bg_b = ( bg >> 24 ) & 0xff;

		let r = ( a * fg_r + ia * bg_r ) >> 8;
		let g = ( a * fg_g + ia * bg_g ) >> 8;
		let b = ( a * fg_b + ia * bg_b ) >> 8;

		( b << 24 ) | ( g << 16 ) | ( r << 8 ) | ( a << 0 )
		*/
//		fg
	}

	pub fn bob( &self, bob_type: BobType ) -> &Bob {
		&self.bobs[ &bob_type ]
	}

	pub fn render_fullscreen_alpha( &self, fb: &mut FB, bobtype: BobType, alpha: f32 )
	{
		let bob = &self.bobs[ &bobtype ];
		let inv_alpha = 1.0 - alpha;

		for it in fb.buffer().iter_mut().zip(bob.data.iter()) {
			let (d, s) = it;
			*d = FB::mix( *s, *d, alpha );
		}		
	}
	pub fn render( &self, fb: &mut FB, bobtype: BobType, x: isize, y: isize ) {
		let bob = &self.bobs[ &bobtype ];
		if x == 0 && y == 0 && bob.width == fb.width && bob.height == fb.height {	// fast blit
//			println!("Fastblit {:?}", bob);
			for it in fb.buffer().iter_mut().zip(bob.data.iter()) {
				let (a, b) = it;
				*a = *b;
			}
			/*
			fb.buffer().iter_mut().zip(&bob.data[..]).for_each(|(dst,&src)|{
				println!("dst {:?}, src {:?}", dst, src );
				*dst = src;
			});
			*/
		} else { // slow blit
			let mut src = 0;
			for r in 0..bob.height {
				if y+r < 0 {
					continue;
				}
				if y + r >= fb.height {
					break;
				}
				let mut dst = ( fb.width * ( r + y ) + x ) as  usize;
				for c in 0..bob.width {
					if x + c >= fb.width {
						break;
					}
					fb.buffer()[dst] = BobManager::blend( bob.data[ src ], fb.buffer()[dst]);
					/*
					let col = bob.data[ src ];
					let a = ( col >> 24 ) as f32 / 255.0;
					let r = 
					let a1 = 1.0 - a;
					let d = fb.buffer()[ dst ];
					fb.buffer()[ dst ] = col * a + d * a1;
					*/
					src += 1;
					dst += 1;
				}
			}
		}
	}
}