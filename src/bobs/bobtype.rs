#[derive(Debug,Eq,PartialEq,Hash,Clone,Copy)]
pub enum BobType {
	None,
	Title,
	GameOver,
	Numbers,
	Target,
	PlayerLeft,
	PlayerRight,
	PlayerUp,
	PlayerDown,
	Background,
	Trash00,
}

