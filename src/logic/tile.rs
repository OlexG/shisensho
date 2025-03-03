#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Tile {
	Blank,
	Number1,
	Number2,
	Number3,
	Number4,
	Number5,
	Number6,
	Number7,
	Number8,
	Number9,
	Sticks1,
	Sticks2,
	Sticks3,
	Sticks4,
	Sticks5,
	Sticks6,
	Sticks7,
	Sticks8,
	Sticks9,
	Circles1,
	Circles2,
	Circles3,
	Circles4,
	Circles5,
	Circles6,
	Circles7,
	Circles8,
	Circles9,
	WindNorth,
	WindEast,
	WindWest,
	WindSouth,
	DragonRedUp,
	DragonRedDown,
	DragonGreen,
	DragonBlue,
}

impl Tile {
	pub const NUM_TILES: usize = 36;
	pub const fn all() -> [Tile; Self::NUM_TILES] {
		use Tile::*;
		[
			Blank,
			Number1,
			Number2,
			Number3,
			Number4,
			Number5,
			Number6,
			Number7,
			Number8,
			Number9,
			Sticks1,
			Sticks2,
			Sticks3,
			Sticks4,
			Sticks5,
			Sticks6,
			Sticks7,
			Sticks8,
			Sticks9,
			Circles1,
			Circles2,
			Circles3,
			Circles4,
			Circles5,
			Circles6,
			Circles7,
			Circles8,
			Circles9,
			WindNorth,
			WindEast,
			WindWest,
			WindSouth,
			DragonRedUp,
			DragonRedDown,
			DragonGreen,
			DragonBlue,
		]
	}

	pub const fn as_char(self) -> char {
		use Tile::*;
		match self {
			Blank => '0',
			Number1 => '1',
			Number2 => '2',
			Number3 => '3',
			Number4 => '4',
			Number5 => '5',
			Number6 => '6',
			Number7 => '7',
			Number8 => '8',
			Number9 => '9',
			Sticks1 => 'A',
			Sticks2 => 'B',
			Sticks3 => 'C',
			Sticks4 => 'D',
			Sticks5 => 'E',
			Sticks6 => 'F',
			Sticks7 => 'G',
			Sticks8 => 'H',
			Sticks9 => 'I',
			Circles1 => 'a',
			Circles2 => 'b',
			Circles3 => 'c',
			Circles4 => 'd',
			Circles5 => 'e',
			Circles6 => 'f',
			Circles7 => 'g',
			Circles8 => 'h',
			Circles9 => 'i',
			WindNorth => '^',
			WindEast => '>',
			WindWest => '<',
			WindSouth => 'v',
			DragonRedUp => '{',
			DragonRedDown => '}',
			DragonGreen => '$',
			DragonBlue => '&',
		}
	}
}

impl std::fmt::Display for Tile {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		use std::fmt::Write;
		formatter.write_char(self.as_char())
	}
}
