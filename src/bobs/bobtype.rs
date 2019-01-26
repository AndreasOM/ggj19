#[derive(Debug,Eq,PartialEq,Hash,Clone,Copy)]
pub enum BobType {
	None,
	Title,
	Target,
	PlayerLeft,
	PlayerRight,
	PlayerUp,
	PlayerDown,
	Background,
	Trash00,
}

