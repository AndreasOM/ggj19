
use image::GenericImageView;
use rand::Rng;

use crate::bobs::bobtype::BobType;
use crate::fb::FB;
use crate::input::Input;

use crate::bobs::bobmanager::BobManager;

#[derive(Debug)]
enum State {
	Walking,
	Carrying,
}

#[derive(Debug)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

#[derive(Debug,Clone)]
pub struct Tile {
	bob_type: BobType,
	walkable: bool,
}

impl Tile {
	pub fn new() -> Tile {
		Tile {
			bob_type: BobType::None,
			walkable: false,
		}
	}
}


#[derive(Debug)]
pub struct Game {
	state: State,
	bobmanager: BobManager,
	player_pos: ( f32, f32 ),
	player_direction: Direction,
	trash: Vec< BobType >,
	grid: Vec< Tile >,
	max_trash: usize,

	render_map: bool,
}

const GRID_WIDTH: usize = 30;
const GRID_HEIGHT: usize = 17;

const MAX_PLAYER_X: f32 = ( 480 - 8 ) as f32;
const MAX_PLAYER_Y: f32 = ( 270 - 8 ) as f32;

impl Game {
	pub fn new() -> Game {
		let mut bobmanager = BobManager::new();
		bobmanager.load_all();
		let mut game = Game {
			state: State::Walking,
			bobmanager: bobmanager,
			player_pos: ( 21.0*16.0, 6.0*16.0 ),
			player_direction: Direction::Down,
			trash: Vec::new(),
			grid: vec![Tile{bob_type:BobType::None,walkable:false};GRID_WIDTH*GRID_HEIGHT],
			max_trash: 5,

			render_map: false,
		};

		game.load_grid();
		game
	}

	fn load_grid( &mut self ) {
		let bytes = include_bytes!("./../resources/level_00_map.png");
		let img = image::load_from_memory_with_format( bytes, image::ImageFormat::PNG ).unwrap();
		println!("map dimensions {:?}", img.dimensions());

		img.to_rgba();
		for y in 0..GRID_HEIGHT {
			for x in 0..GRID_WIDTH {
				let p = img.get_pixel( x as u32, y as u32 );
				let pix = ( p.data[ 0 ], p.data[ 1 ], p.data[ 2 ], p.data[ 3 ] );
				match pix {
					( 0, 0, 0, _ ) => self.grid[ y * GRID_WIDTH + x ].walkable = true,
					_ => {},
				}
			}
		}

	}
	fn spawn_trash( &mut self ) {
		// todo find free grid slot
		let x = rand::thread_rng().gen_range(0, GRID_WIDTH - 1);
		let y = rand::thread_rng().gen_range(0, GRID_HEIGHT - 1);

		if self.grid[ y * GRID_WIDTH + x ].bob_type == BobType::None {
			self.grid[ y * GRID_WIDTH + x ].bob_type = BobType::Trash00;
		}
	}

	fn pos_to_grid( &self, x: f32, y: f32 ) -> ( isize, isize ) {
		( ( x/16.0 ).floor() as isize, ( y/16.0 ).floor() as isize )
	}
	fn is_pos_walkable( &self, x: f32, y: f32 ) -> bool {
		if x <= 8.0 || y <= 8.0 || x >= MAX_PLAYER_X || y >= MAX_PLAYER_Y {
			return false;
		}
		let ( gx, gy ) = self.pos_to_grid( x, y );
//		let gx = ( x/16.0 ).floor() as isize;
//		let gy = ( y/16.0 ).floor() as isize;

//		println!("{:?} {:?}", x, gx );
		if /*gx < 0 || gy < 0 || */gx >= GRID_WIDTH as isize || gy >= GRID_HEIGHT as isize {
			return false;
		}

		self.grid[ gy as usize * GRID_WIDTH + gx as usize ].walkable

//		true
	}
	fn grid_in_front_of_player( &self ) -> ( isize, isize ) {
		let ( x, y ) = self.pos_to_grid(self.player_pos.0 + 8.0 , self.player_pos.1 + 8.0 );
		let ( x, y ) = match self.player_direction {
			Direction::Right => ( x + 1, y ),
			Direction::Left => ( x - 1, y ),
			Direction::Up => ( x, y - 1 ),
			Direction::Down => ( x, y + 1 ),
		};
		( x, y )
	}

	pub fn update( &mut self, input: &mut Input ) {

		if input.action_a && self.trash.len() < self.max_trash {
			let ( fx, fy ) = self.grid_in_front_of_player();
			if fx >= 0 && fy >= 0 {
				let fx = fx as usize;
				let fy = fy as usize;
				if self.grid[ fy * GRID_WIDTH + fx ].bob_type != BobType::None {
					self.trash.push( self.grid[ fy * GRID_WIDTH + fx ].bob_type );
					self.grid[ fy * GRID_WIDTH + fx ].bob_type = BobType::None;
				}
			}

			self.state = State::Carrying;
		} else if input.action_b && self.trash.len() > 0 {
			let ( fx, fy ) = self.grid_in_front_of_player();
			if fx >= 0 && fy >= 0 {
				let fx = fx as usize;
				let fy = fy as usize;
				if self.grid[ fy * GRID_WIDTH + fx ].bob_type == BobType::None {
//					self.trash.push( self.grid[ fy * GRID_WIDTH + fx ].bob_type );	// :TODO: pop last
					self.grid[ fy * GRID_WIDTH + fx ].bob_type = BobType::Trash00;
				}
			}

		} else {
			self.state = State::Walking;
		}

		let dist = 1.0;
		let old_pos = self.player_pos;

		if input.right {
			self.player_pos.0 += dist;
			self.player_direction = Direction::Right;
		} else if input.left {
			self.player_pos.0 -= dist;
			self.player_direction = Direction::Left;
		} else if input.down {
			self.player_pos.1 += dist;
			self.player_direction = Direction::Down;
		} else if input.up {
			self.player_pos.1 -= dist;
			self.player_direction = Direction::Up;
		}

		if !self.is_pos_walkable( self.player_pos.0 + 8.0 , self.player_pos.1+ 8.0 ) {
			self.player_pos = old_pos;
		}
/*
		if self.player_pos.0 < 0.0 {
			self.player_pos.0 = 0.0;
		}
		if self.player_pos.0 > MAX_PLAYER_X {
			self.player_pos.0 = MAX_PLAYER_X;
		}
		if self.player_pos.1 < 0.0 {
			self.player_pos.1 = 0.0;
		}
		if self.player_pos.1 > MAX_PLAYER_Y {
			self.player_pos.1 = MAX_PLAYER_Y;
		}
*/

		// debug
		if input.debug_0 {
			input.debug_0 = false;
			self.spawn_trash();
		}
		if input.debug_1 {
			input.debug_1 = false;
			self.render_map = true;
		} else {
			self.render_map = false;
		}
	}

	pub fn render( &mut self, fb: &mut FB) {
		let mut col: u32 = 0xf1;
		match self.state {
			State::Carrying => col = 0x1f,
			_ => {},
		}
 		for i in fb.buffer().iter_mut() {
			*i = col; // write something more funny here!
		}

		self.bobmanager.render( fb, BobType::Background, 0, 0 );

		let player_bob = match self.player_direction {
			Direction::Up => BobType::PlayerUp,
			Direction::Right => BobType::PlayerRight,
			Direction::Down => BobType::PlayerDown,
			Direction::Left => BobType::PlayerLeft,
		};

//		println!("{:?}", self.player_pos );
		self.bobmanager.render( fb, player_bob, self.player_pos.0 as usize, self.player_pos.1 as usize );

		let ( fx, fy ) = self.grid_in_front_of_player();
		if fx >= 0 && fy >= 0 {
			self.bobmanager.render( fb, BobType::Target, ( fx * 16 ) as usize, ( fy * 16 ) as usize );
		}

		for y in 0..GRID_HEIGHT {
			for x in 0..GRID_WIDTH {
				let p = y * GRID_WIDTH + x;
				if self.grid[ p ].bob_type != BobType::None {
					self.bobmanager.render( fb, self.grid[ p ].bob_type, x*16, y*16 );
				}
			}
		}
		/*
		for t in &self.trash {
			self.bobmanager.render( fb, t.bob_type, t.pos.0 as usize, t.pos.1 as usize );
		};
		*/

		if self.render_map {
			for y in 0..GRID_HEIGHT {
				for x in 0..GRID_WIDTH {
					let p = y * GRID_WIDTH + x;
					if self.grid[ p ].walkable {
						fb.fill_rect( x*16, y*16, ( x+1 )*16, ( y+1 )*16, 0xaa33aaff );
					} else {
						fb.fill_rect( x*16, y*16, ( x+1 )*16, ( y+1 )*16, 0x00330000 );
					}
				}
			}
			let ( px, py ) = self.pos_to_grid( self.player_pos.0, self.player_pos.1 );
			let px = px as usize;
			let py = py as usize;

			fb.fill_rect( px*16, py*16, ( px+1 )*16, ( py+1 )*16, 0xffffffff );

			let ( px, py ) = self.grid_in_front_of_player();
			let px = px as usize;
			let py = py as usize;
			fb.fill_rect( px*16, py*16, ( px+1 )*16, ( py+1 )*16, 0x88ff88ff );

			let px = self.player_pos.0 as usize;
			let py = self.player_pos.1 as usize;
			fb.fill_rect( px, py, ( px+1 ), ( py+1 ), 0x333333ff );

		}
	}
}

