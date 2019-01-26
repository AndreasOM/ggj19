
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

#[derive(Debug)]
pub struct Trash {
	pos: ( f32, f32 ),
	bob_type: BobType,
}

#[derive(Debug)]
pub struct Game {
	state: State,
	bobmanager: BobManager,
	player_pos: ( f32, f32 ),
	player_direction: Direction,
	trash: Vec< Trash >,
	grid: Vec< BobType >,
}

const GRID_WIDTH: usize = 30;
const GRID_HEIGHT: usize = 17;

const MAX_PLAYER_X: f32 = ( 480 - 16 ) as f32;
const MAX_PLAYER_Y: f32 = ( 270 - 16 ) as f32;

impl Game {
	pub fn new() -> Game {
		let mut bobmanager = BobManager::new();
		bobmanager.load_all();
		Game {
			state: State::Walking,
			bobmanager: bobmanager,
			player_pos: ( 0.0, 0.0 ),
			player_direction: Direction::Down,
			trash: Vec::new(),
			grid: vec![BobType::None;GRID_WIDTH*GRID_HEIGHT],
		}
	}

	fn spawn_trash( &mut self ) {
		// todo find free grid slot
		let x = rand::thread_rng().gen_range(0, GRID_WIDTH - 1);
		let y = rand::thread_rng().gen_range(0, GRID_HEIGHT - 1);

		if self.grid[ y * GRID_WIDTH + x ] == BobType::None {
			self.grid[ y * GRID_WIDTH + x ] = BobType::Trash00;
		}
	}
	pub fn update( &mut self, input: &mut Input ) {

		if input.action_a {
			self.state = State::Carrying;
		} else {
			self.state = State::Walking;
		}

		let dist = 1.0;
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


		// debug
		if input.debug_0 {
			input.debug_0 = false;
			self.spawn_trash();
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

		self.bobmanager.render( fb, player_bob, self.player_pos.0 as usize, self.player_pos.1 as usize );

		for y in 0..GRID_HEIGHT {
			for x in 0..GRID_WIDTH {
				let p = y * GRID_WIDTH + x;
				if self.grid[ p ] != BobType::None {
					self.bobmanager.render( fb, self.grid[ p ], x*16, y*16 );
				}
			}
		}
		/*
		for t in &self.trash {
			self.bobmanager.render( fb, t.bob_type, t.pos.0 as usize, t.pos.1 as usize );
		};
		*/

	}
}

