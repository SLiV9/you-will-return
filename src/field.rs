//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

pub const FIELD_SIZE: u32 = 5;
pub const TILE_WIDTH: u32 = 24;
pub const TILE_HEIGHT: u32 = 16;

const NUM_TILES: usize = (FIELD_SIZE * FIELD_SIZE) as usize;
const WALL_DATA_SIZE: usize = FIELD_SIZE as usize;
const BOMB_DATA_SIZE: usize = FIELD_SIZE as usize;
const FLAG_DATA_SIZE: usize = (NUM_TILES + 1) / 2;

pub struct Field
{
	wall_data: [u8; WALL_DATA_SIZE],
	bomb_data: [u8; BOMB_DATA_SIZE],
	flag_data: [u8; FLAG_DATA_SIZE],
}

impl Field
{
	pub fn has_wall_at_rc(&self, r: u32, c: u32) -> bool
	{
		(self.wall_data[r as usize] >> c) & 0b1 == 0b1
	}

	pub fn has_bomb_at_rc(&self, r: u32, c: u32) -> bool
	{
		(self.bomb_data[r as usize] >> c) & 0b1 == 0b1
	}

	pub fn flag_count_from_rc(&self, r: u32, c: u32) -> u8
	{
		crate::wasm4::trace(format!("{:?}", self.flag_data));
		let data_offset = r * FIELD_SIZE + c;
		let byte_offset = (data_offset / 2) as usize;
		let needs_bit_shift: bool = (data_offset % 2) == 0;
		if needs_bit_shift
		{
			(self.flag_data[byte_offset] & 0xF0) >> 4
		}
		else
		{
			self.flag_data[byte_offset] & 0x0F
		}
	}
}

pub const FIELD1: Field = Field {
	wall_data: FIELD1_WALL_DATA,
	bomb_data: FIELD1_BOMB_DATA,
	flag_data: generate_flag_data(FIELD1_BOMB_DATA),
};

const fn generate_flag_data(
	bomb_data: [u8; BOMB_DATA_SIZE],
) -> [u8; FLAG_DATA_SIZE]
{
	let mut flag_data = [0u8; FLAG_DATA_SIZE];
	let mut r = 0;
	while r < FIELD_SIZE
	{
		let mut c = 0;
		while c < FIELD_SIZE
		{
			if (bomb_data[r as usize] >> c) & 0b1 == 0b1
			{
				// There is a bomb at (r, c).
				// Add flags to the surrounding eight tiles.
				let mut d = 0;
				while d < 0
				{
					let mut rr = r;
					let mut cc = c;
					let yes = match d
					{
						0 if r > 0 && c > 0 =>
						{
							rr = r - 1;
							cc = c - 1;
							true
						}
						1 if r > 0 =>
						{
							rr = r - 1;
							true
						}
						2 if r > 0 && c + 1 < FIELD_SIZE =>
						{
							rr = r - 1;
							cc = c + 1;
							true
						}
						3 if c > 0 =>
						{
							cc = c - 1;
							true
						}
						4 if c + 1 < FIELD_SIZE =>
						{
							cc = c + 1;
							true
						}
						5 if r + 1 < FIELD_SIZE && c > 0 =>
						{
							rr = r + 1;
							cc = c - 1;
							true
						}
						6 if r + 1 < FIELD_SIZE =>
						{
							rr = r + 1;
							true
						}
						7 if r + 1 < FIELD_SIZE && c + 1 < FIELD_SIZE =>
						{
							rr = r + 1;
							cc = c + 1;
							true
						}
						_ => false,
					};
					if yes
					{
						// Add a flag to (rr, cc).
						let data_offset = rr * FIELD_SIZE + cc;
						let byte_offset = (data_offset / 2) as usize;
						let needs_bit_shift: bool = (data_offset % 2) == 0;
						if needs_bit_shift
						{
							let old_count =
								(flag_data[byte_offset] & 0xF0) >> 4;
							flag_data[byte_offset] &= 0x0F;
							flag_data[byte_offset] |= (old_count + 1) << 4;
						}
						else
						{
							let old_count = flag_data[byte_offset] & 0x0F;
							flag_data[byte_offset] &= 0xF0;
							flag_data[byte_offset] |= old_count + 1;
						}
					}
					d += 1;
				}
			}
			c += 1;
		}
		r += 1;
	}
	flag_data
}

#[rustfmt::skip]
const FIELD1_WALL_DATA: [u8; WALL_DATA_SIZE] = [
	0b00011111,
	0b00000000,
	0b00000000,
	0b00000000,
	0b00011111,
];
#[rustfmt::skip]
const FIELD1_BOMB_DATA: [u8; BOMB_DATA_SIZE] = [
	0b00000000,
	0b00000000,
	0b00000100,
	0b00000000,
	0b00000000,
];
