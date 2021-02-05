pub trait Digest {
    fn into_bytes(self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct DigestError;

#[derive(Debug)]
pub struct Hex(Vec<u8>);

impl Hex {
    pub fn new(source: &str) -> Result<Self, DigestError> {
	let bytes: Result<Vec<u8>, _> = source
	    .as_bytes()
	    .chunks(2)
	    .map(|bs| std::str::from_utf8(bs).unwrap())
	    .map(|frag| u8::from_str_radix(frag, 16))
	    .collect();

	match bytes {
	    Ok(b) => Ok(Hex(b)),
	    _ => Err(DigestError)
	}
    }
}

impl Digest for Hex {
    fn into_bytes(self) -> Vec<u8> {
	self.0
    }
}

#[derive(Debug)]
pub struct Base64(Vec<u8>);

impl Base64 {
    pub fn new(source: &str) -> Result<Self, DigestError> {
	let bytes = base64::decode(source).map_err(|_| DigestError)?;
	Ok(Base64(bytes))
    }
}

impl Digest for Base64 {
    fn into_bytes(self) -> Vec<u8> {
	self.0
    }
}

#[derive(Debug)]
enum Region {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight
}

#[derive(Debug)]
enum Direction {
    Stay,
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    SouthWest,
    South,
    SouthEast
}

#[derive(Debug, Clone, Copy)]
enum Bits {
    ZZ,
    ZO,
    OZ,
    OO
}

impl Bits {
    fn from_bits(bits: u8) -> Self {
	use Bits::*;
	
	match bits {
	    0 => ZZ,
	    1 => ZO,
	    2 => OZ,
	    3 => OO,
	    _ => unimplemented!()
	}
    }

    fn from_byte(byte: u8) -> [Self; 4] {
	[
	    Self::from_bits(byte & 3),
	    Self::from_bits((byte >> 2) & 3),
	    Self::from_bits((byte >> 4) & 3),
	    Self::from_bits((byte >> 6) & 3),
	]
    }
}

#[derive(Default)]
struct Entry(i8);

impl Entry {
    const FMT: [char; 17] = [' ', '.', 'o', '+', '=', '*', 'B', 'O', 'X', '@', '%', '&', '#', '/', '^', 'S', 'E'];

    fn incr(&mut self) {
	self.0 += 1
    }

    fn as_char(&self) -> char {
	if self.0 < 17 {
	    Self::FMT[self.0 as usize]
	} else {
	    Self::FMT[16]
	}
    }
}

struct RandomArt {
    board: [[Entry; 17]; 9],
    pos: (usize, usize)
}

impl Default for RandomArt {
    fn default() -> Self {
	Self {
	    board: Default::default(),
	    pos: Self::start_position()
	}
    }
}

impl RandomArt {
    fn start_position() -> (usize, usize) {
	(4, 8)
    }

    fn state(&self) -> Region {
	let (row, col) = self.pos;
	match (row, col) {
	    (0, 0) => Region::TopLeft,
	    (0, 16) => Region::TopRight,
	    (0, _) => Region::Top,
	    (8, 0) => Region::BottomLeft,
	    (8, 16) => Region::BottomRight,
	    (8, _) => Region::Bottom,
	    (_, 0) => Region::Left,
	    (_, 16) => Region::Right,
	    _ => Region::Center
	}
    }

    fn move_direction(&self, bits: Bits) -> Direction {
	use Region::*;
	use Bits::*;
	use Direction::*;

	match (self.state(), bits) {
	    (TopLeft, ZZ) => Stay,
	    (TopLeft, ZO) => East,
	    (TopLeft, OZ) => South,
	    (TopLeft, OO) => SouthEast,
	    (Top, ZZ) => West,
	    (Top, ZO) => East,
	    (Top, OZ) => SouthWest,
	    (Top, OO) => SouthEast,
	    (TopRight, ZZ) => West,
	    (TopRight, ZO) => Stay,
	    (TopRight, OZ) => SouthWest,
	    (TopRight, OO) => South,
	    (Left, ZZ) => North,
	    (Left, ZO) => NorthEast,
	    (Left, OZ) => South,
	    (Left, OO) => SouthEast,
	    (Right, ZZ) => NorthWest,
	    (Right, ZO) => North,
	    (Right, OZ) => SouthWest,
	    (Right, OO) => South,
	    (Center, ZZ) => NorthWest,
	    (Center, ZO) => NorthEast,
	    (Center, OZ) => SouthWest,
	    (Center, OO) => SouthEast,
	    (BottomLeft, ZZ) => North,
	    (BottomLeft, ZO) => NorthEast,
	    (BottomLeft, OZ) => Stay,
	    (BottomLeft, OO) => East,
	    (Bottom, ZZ) => NorthWest,
	    (Bottom, ZO) => NorthEast,
	    (Bottom, OZ) => West,
	    (Bottom, OO) => East,
	    (BottomRight, ZZ) => NorthWest,
	    (BottomRight, ZO) => North,
	    (BottomRight, OZ) => West,
	    (BottomRight, OO) => Stay,
	}
    }

    fn new_position(&self, direction: &Direction) -> (usize, usize) {
	use Direction::*;
	
	let (row, col) = self.pos;
	match direction {
	    Stay => (row, col),
	    NorthWest => (row - 1, col - 1),
	    North => (row - 1, col),
	    NorthEast => (row - 1, col + 1),
	    West => (row, col - 1),
	    East => (row, col + 1),
	    SouthWest => (row + 1, col - 1),
	    South => (row + 1, col),
	    SouthEast => (row + 1, col + 1)
	}
    }

    fn incr(&mut self) {
	self.board[self.pos.0][self.pos.1].incr();
    }
}

pub fn randomart(fingerprint: impl Digest) {

    let mut art = RandomArt::default();
    let bytes = fingerprint.into_bytes();

    for byte in bytes {
	for bits in Bits::from_byte(byte).iter() {
	    let direction = art.move_direction(*bits);
	    art.pos = art.new_position(&direction);
	    art.incr();
	}
    }

    
    println!("+-----------------+");
    for (i, row) in art.board.iter().enumerate() {
	print!("|");
	for (j, col) in row.iter().enumerate() {
	    let c = match (i, j) {
		(4, 8) => 'S',
		_ if (i, j) == art.pos => 'E',
		_ => col.as_char()
	    };
	    print!("{}", c);
	}
	println!("|");
    }
    println!("+-----------------+");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
	eprintln!("{:b}, {:?}", 148, Bits::from_byte(148));
    }
}
