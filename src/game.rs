
use image::GenericImageView;
use rand::Rng;
use perlin::PerlinNoise;

use crate::bobs::bobtype::BobType;
use crate::counter::Counter;
use crate::fb::FB;
use crate::input::Input;

use crate::bobs::bobmanager::BobManager;

#[derive(Debug)]
enum State {
	Walking,
	Rowing,
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
	rowable: bool,
	water: bool,
	trash: bool,	// actually trash can
}

impl Tile {
	pub fn new() -> Tile {
		Tile {
			bob_type: BobType::None,
			walkable: false,
			rowable: false,
			water: false,
			trash: false,
		}
	}

	// RGBA
	pub fn from_color( &mut self, col: &[u8] ) {
		let pix = ( col[ 0 ], col[ 1 ], col[ 2 ], col[ 3 ] );
		match pix {
			( 0x00, 0x00, 0x00, _ ) => self.walkable = true,
			( 0x00, 0x00, 0xff, _ ) => { self.rowable = true; self.water = true },
			( 0x00, 0xff, 0x00, _ ) => { self.trash = true },
			( 0xff, 0xff, 0x00, _ ) => { self.walkable = true; self.rowable = true },
			( 0xff, 0xff, 0xff, _ ) => {  }, // blocked

			_ => {},
		}
	}

	// BGRA
	// ARGB	-> WTF?
	pub fn to_color( &self ) -> u32 {
		match ( self.walkable, self.rowable, self.trash ) {
			( true, true, false )	=> 0x80ffff00,	// slip
			( true, false, false )	=> 0x80333333,	// land
			( false, true, false )	=> 0x800000ff,	// water
			( false, false, false )	=> 0x80fffff,	// blocked
			( _, _, true )			=> 0x8000ff00,	// trash
			_						=> 0x80ff00ff,	// wtf
		}
	}
}

impl Default for Tile {
	fn default() -> Tile {
		Tile {
			bob_type: BobType::None,
			walkable: false,
			rowable: false,
			water: false,
			trash: false,
		}		
	}
}


#[derive(Debug)]
pub struct Game {
	title_overlay: f32,
	state: State,
	bobmanager: BobManager,
	player_pos: ( f32, f32 ),
	player_direction: Direction,
	trash: Vec< BobType >,
	grid: Vec< Tile >,
	max_trash_bag: usize,
	bag_fill: f32,
	bag_fill_target: f32,
	time: f32,
	trash_recycled: usize,
	time_since_last_trash: f32,
	active_trash: usize,
	max_trash: usize,
	gameover: bool,
	gameover_offset: f32,

	render_map: bool,
}

const GRID_WIDTH: isize = 30;
const GRID_HEIGHT: isize = 17;

const MAX_PLAYER_X: f32 = ( 480 - 8 ) as f32;
const MAX_PLAYER_Y: f32 = ( 270 - 8 ) as f32;

impl Game {
	pub fn new() -> Game {
		let mut bobmanager = BobManager::new();
		bobmanager.load_all();
		let mut game = Game {
			title_overlay: 1.0,
			state: State::Walking,
			bobmanager: bobmanager,
			player_pos: ( 21.0*16.0, 6.0*16.0 ),
			player_direction: Direction::Down,
			trash: Vec::new(),
//			grid: vec![Tile{bob_type:BobType::None,walkable:false};GRID_WIDTH*GRID_HEIGHT],
			grid: vec![Tile{..Default::default()};( GRID_WIDTH*GRID_HEIGHT ) as usize],
			
			max_trash_bag: 5,
			bag_fill: 0.0,
			bag_fill_target: 0.0,
			time: 0.0,
			trash_recycled: 0,
			time_since_last_trash: 0.0,
			active_trash: 0,
			max_trash: 100,
			gameover: false,
			gameover_offset: 1.0,

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

				self.grid[ ( y * GRID_WIDTH + x ) as usize ].from_color( &p.data );
			}
		}

	}
	fn spawn_trash( &mut self ) {
		// todo find free grid slot
		let x = rand::thread_rng().gen_range(0, GRID_WIDTH - 1);
		let y = rand::thread_rng().gen_range(0, GRID_HEIGHT - 1);

		let p = ( y * GRID_WIDTH + x ) as usize;
		let t = &self.grid[ p ];
		if t.bob_type == BobType::None && ( t.walkable || t.rowable ){

			self.grid[ p ].bob_type = BobType::Trash00;
			self.time_since_last_trash = 0.0;
			self.active_trash += 1;
		}
	}

	fn pos_to_grid( &self, x: f32, y: f32 ) -> ( isize, isize ) {
		( ( x/16.0 ).floor() as isize, ( y/16.0 ).floor() as isize )
	}
	fn is_accessable( &self, x: f32, y: f32 ) -> bool {
		if x <= 8.0 || y <= 8.0 || x >= MAX_PLAYER_X || y >= MAX_PLAYER_Y {
			return false;
		}

		true		
	}
	fn is_pos_walkable( &self, x: f32, y: f32 ) -> bool {
		if !self.is_accessable(x, y) {
			return false;
		}
		let ( gx, gy ) = self.pos_to_grid( x, y );
		self.grid[ ( gy * GRID_WIDTH + gx ) as usize ].walkable
	}
	fn is_pos_rowable( &self, x: f32, y: f32 ) -> bool {
		if !self.is_accessable(x, y) {
			return false;
		}
		let ( gx, gy ) = self.pos_to_grid( x, y );
		self.grid[ ( gy  * GRID_WIDTH + gx ) as usize ].rowable
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

	fn tile_in_fron_of_player( &mut self ) -> &mut Tile {
		let ( x, y ) = self.grid_in_front_of_player();
		let x = x as isize;
		let y = y as isize;

		let p = ( y * GRID_WIDTH + x ) as usize;

		&mut self.grid[ p ]
	}

	fn tile( &self, p: isize ) -> &Tile {
		if p < 0 || p as usize > self.grid.len() {
			println!("Warning tried to access tile {:?}", p );
			return &self.grid[ 0 ];
		}

		&self.grid[ p as usize ]
	}

	fn tile_at( &self, x: isize, y: isize ) -> &Tile {
		let p = y * GRID_WIDTH + x;

		self.tile( p )
	}

	fn mut_tile( &mut self, p: isize ) -> &mut Tile {
		if p < 0 || p as usize > self.grid.len() {
			println!("Warning tried to access tile {:?}", p );
			return &mut self.grid[ 0 ];
		}

		&mut self.grid[ p as usize ]
	}

	fn mut_tile_at( &mut self, x: isize, y: isize ) -> &mut Tile {
		let p = y * GRID_WIDTH + x;

		self.mut_tile( p )
	}

	pub fn restart( &mut self ) {
		self.title_overlay = 0.0;
		self.state =  State::Walking;
		self.player_pos = ( 21.0*16.0, 6.0*16.0 );
		self.player_direction = Direction::Down;
		self.trash = Vec::new();
		for t in &mut self.grid {
			t.bob_type = BobType::None;
		}
//		self.grid = vec![Tile{..Default::default()};( GRID_WIDTH*GRID_HEIGHT ) as usize];
		self.max_trash_bag = 5;
		self.bag_fill = 0.0;
		self.bag_fill_target = 0.0;
		self.time = 0.0;
		self.trash_recycled = 0;
		self.time_since_last_trash = 0.0;
		self.active_trash = 0;
		self.max_trash = 100;
		self.gameover = false;
		self.gameover_offset = 1.0;

		self.render_map = false;
	}

	// 0.0 -> easy
	// 100.0 -> super hard
	fn difficulty( &self ) -> f32 {
		let trash_factor = if self.trash_recycled > 0 {
			0.2*( ( self.trash_recycled as f32 ).log(3.0) )
		} else {
			0.0
		};

		100.0 *  trash_factor
	}

	pub fn update( &mut self, time_step: f32, input: &mut Input ) {



		if self.title_overlay < 1.0 {
			// game running
			self.time += time_step;
			self.time_since_last_trash += time_step;

			if self.title_overlay > 0.0 {
				self.title_overlay -= 0.3 * time_step;
			} else {
				self.title_overlay = 0.0;
			}
		} else {
			if input.any {
				self.title_overlay -= 0.001;
			}
		}

		if !self.gameover && self.active_trash > self.max_trash {
			self.gameover = true;
			self.gameover_offset = 1.0;
		}

		if self.gameover {
			self.gameover_offset -= 0.75 * time_step;
			if self.gameover_offset < 0.0 {
				self.gameover_offset = 0.0;
				if input.any {
					self.restart();
				}
			}
			return;
		}

		let difficulty = self.difficulty();
		let mut auto_spawn_time = 3.0 - 2.0 * ( difficulty/100.0 );
		if auto_spawn_time < 0.1 {
			auto_spawn_time = 0.1;
		}
		if self.time_since_last_trash > auto_spawn_time {
			self.spawn_trash();
		}

		if input.action_a && self.trash.len() < self.max_trash_bag {
			let ( fx, fy ) = self.grid_in_front_of_player();
			if fx >= 0 && fy >= 0 {
				let fx = fx as isize;
				let fy = fy as isize;
				let t = self.tile_at( fx, fy );
				let p = ( fy * GRID_WIDTH + fx ) as usize;
//				if self.grid[ p ].bob_type != BobType::None {
				if t.bob_type != BobType::None {
//					self.trash.push( self.grid[ p ].bob_type );
					self.trash.push( t.bob_type );
					let t = self.mut_tile_at( fx, fy );
					t.bob_type = BobType::None;
//					self.grid[ p ].bob_type = BobType::None;
					self.active_trash -= 1;
				}
			}
		} else if input.action_b && self.trash.len() > 0 {
			input.action_b = false;
			let bob_type = self.trash[ self.trash.len() - 1 ].clone();
			let mut trash_dropped = false;
			let target_tile = self.tile_in_fron_of_player();
			if target_tile.trash {
				trash_dropped = true;
				self.trash_recycled += 1;
			} else if target_tile.bob_type == BobType::None {
				if target_tile.walkable || target_tile.rowable {
					target_tile.bob_type = bob_type;
					trash_dropped = true;
					self.active_trash += 1;
				}
			}

			if trash_dropped {
				self.trash.pop();
			}
		} else {
		}

		let dist = 96.0*time_step;
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

		let mut new_is_valid = true;
		let is_walkable = self.is_pos_walkable( self.player_pos.0 + 8.0 , self.player_pos.1 + 8.0 );
		let is_rowable = self.is_pos_rowable( self.player_pos.0 + 8.0 , self.player_pos.1 + 8.0 );
		match self.state {
			State::Walking => {
				if !is_walkable {
					if is_rowable {
						let old_is_rowable = self.is_pos_rowable( old_pos.0 + 8.0 , old_pos.1 + 8.0 );
						if !old_is_rowable {
							new_is_valid = false;
						} else {
							self.state = State::Rowing;
						}
					} else {
						new_is_valid = false;
					}
				}				
			}
			State::Rowing => {
				if !is_rowable {
					if is_walkable {
						let old_is_walkable = self.is_pos_walkable( old_pos.0 + 8.0 , old_pos.1 + 8.0 );
						if !old_is_walkable {
							new_is_valid = false;
						} else {
							self.state = State::Walking;
						}
					} else {
						new_is_valid = false;
					}
				}				
			}
		}

		if !new_is_valid {
			self.player_pos = old_pos;
		}

		// bag

		self.bag_fill_target = 16.0 * self.trash.len() as f32;

		let bag_fill_delta = self.bag_fill_target - self.bag_fill;
		let bag_fill_sign = bag_fill_delta.signum();

		let bag_fill_speed = 64.0 * time_step;
		if bag_fill_delta > 0.0 {
			self.bag_fill += bag_fill_speed;
			if self.bag_fill > self.bag_fill_target {
				self.bag_fill = self.bag_fill_target;
			}
		} else {
			self.bag_fill -= bag_fill_speed;
			if self.bag_fill < self.bag_fill_target {
				self.bag_fill = self.bag_fill_target;
			}
		}

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
		self.bobmanager.render( fb, BobType::Background, 0, 0 );

		let player_bob = match self.player_direction {
			Direction::Up => BobType::PlayerUp,
			Direction::Right => BobType::PlayerRight,
			Direction::Down => BobType::PlayerDown,
			Direction::Left => BobType::PlayerLeft,
		};

//		println!("{:?}", self.player_pos );
		self.bobmanager.render( fb, player_bob, self.player_pos.0 as isize, self.player_pos.1 as isize );

		let ( fx, fy ) = self.grid_in_front_of_player();
		if fx >= 0 && fy >= 0 {
			self.bobmanager.render( fb, BobType::Target, ( fx * 16 ) as isize, ( fy * 16 ) as isize );
		}

		let perlin = PerlinNoise::new();
		for y in 0..GRID_HEIGHT {
			for x in 0..GRID_WIDTH {
				let p = ( y * GRID_WIDTH + x ) as usize;
				if self.grid[ p ].bob_type != BobType::None {
//					let ox = ( perlin.get3d([x as f64, y as f64, self.time as f64 * 0.01]) * 8.0 ) as usize;
//					let oy = ( perlin.get3d([x as f64, y as f64, self.time as f64 * 0.011]) * 8.0 ) as usize;
					let ox = ( 0.0 + ( x as f64 + y as f64 + self.time as f64 * 11.1 ).sin() * 2.0 ) as isize;
					let oy = -ox;
					let bx = ( ( ( x * 16 ) as isize ) + ox ) as isize;
					let by = ( ( ( y * 16 ) as isize ) + oy ) as isize;
					if self.grid[ p ].rowable {
						self.bobmanager.render( fb, self.grid[ p ].bob_type, bx, by );
					} else {
						self.bobmanager.render( fb, self.grid[ p ].bob_type, x*16, y*16 );
					}
				}
			}
		}

		// bag
		let bag_bottom = 270 - 16;
		fb.fill_rect( 16, bag_bottom-( self.bag_fill as isize ), 32, bag_bottom, 0x00000ff );

		// ui
		let number_bob = self.bobmanager.bob( BobType::Numbers );
//		self.trash_recycled += 1;
		Counter::draw( self.trash_recycled, &number_bob.data, 480-32, 4, fb );
		self.bobmanager.render( fb, BobType::Trash00, 480-16, 4 );

		// trash bar
		let mut trash_percentage = self.active_trash as f32 / self.max_trash as f32;
		if trash_percentage > 1.0 {
			trash_percentage = 1.0;
		}
		let r = ( trash_percentage * 255.0 ) as u32;
		let g = ( ( 1.0-trash_percentage ) * 255.0 ) as u32;
		let b = 0x20; //( trash_percentage * 255.0 ) as u32;
		let mut trash_color =  ( 0xff << 24 ) | ( r << 16 ) | ( g << 8 ) | ( b << 0 );
		if self.gameover {
			trash_color = 0xffff0000;
		}
		fb.fill_rect( 480-16, 270 - 24 -( trash_percentage * 144.0) as isize, 480-8 , 270 - 24, trash_color );

		// debug
		if self.render_map {
			for y in 0..GRID_HEIGHT {
				for x in 0..GRID_WIDTH {
					let p = ( y * GRID_WIDTH + x ) as usize;
					let col = self.grid[ p ].to_color();
					fb.fill_rect( x*16, y*16, ( x+1 )*16, ( y+1 )*16, col );
				}
			}
			let ( px, py ) = self.pos_to_grid( self.player_pos.0, self.player_pos.1 );
			let px = px as isize;
			let py = py as isize;

			fb.fill_rect( px*16, py*16, ( px+1 )*16, ( py+1 )*16, 0xffffffff );

			let ( px, py ) = self.grid_in_front_of_player();
			let px = px as isize;
			let py = py as isize;
			fb.fill_rect( px*16, py*16, ( px+1 )*16, ( py+1 )*16, 0x88ff88ff );

			let px = self.player_pos.0 as isize;
			let py = self.player_pos.1 as isize;
			fb.fill_rect( px, py, ( px+1 ), ( py+1 ), 0x333333ff );

		}

		if self.title_overlay > 0.0 {
			self.bobmanager.render_fullscreen_alpha( fb, BobType::Title, self.title_overlay );
		}


		if self.gameover {
			let gameover_bob = self.bobmanager.bob( BobType::GameOver );
			let offset = ( 0.5 * ( 1.0 - self.gameover_offset ) * ( gameover_bob.height as f32 ) ) as isize * gameover_bob.width;
//			println!("offset {:?} {:?}", offset, self.gameover_offset );
			fb.blit_rect( 0, 0, fb.width, fb.height, &gameover_bob.data, offset, gameover_bob.width );
		}

	}
}

