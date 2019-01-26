
pub struct Input {
	pub action_a: bool,
	pub right: bool,
	pub left: bool,
	pub up: bool,
	pub down: bool,

	pub debug_0: bool,
}

impl Input {
	pub fn new() -> Input {
		Input {
			action_a: false,
			right: false,
			left: false,
			up: false,
			down: false,

			debug_0: false,
		}
	}

	pub fn update_keys( &mut self, maybe_keys: &Option< Vec< minifb::Key > > ) {
		match maybe_keys {
			Some( keys ) => {
				self.action_a = false;
				self.right = false;
				self.left = false;
				self.up = false;
				self.down = false;

				for k in keys {
					match k {
						minifb::Key::S => self.action_a = true,
						minifb::Key::Right => self.right = true,
						minifb::Key::Left => self.left = true,
						minifb::Key::Up => self.up = true,
						minifb::Key::Down => self.down = true,

						minifb::Key::Key0 => self.debug_0 = true,
						_ => {},
					}
				}
			},
			_ => {},
		}
	}
}

