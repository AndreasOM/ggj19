#[derive(Debug,Eq,PartialEq,Hash,Clone,Copy)]
pub enum BobType {
	None,
	Target,
	PlayerLeft,
	PlayerRight,
	PlayerUp,
	PlayerDown,
	Background,
	Trash00,
}

