//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

pub const FIELD_WIDTH: u8 = 8;
pub const FIELD_HEIGHT: u8 = 5;

const NUM_TILES: usize = (FIELD_WIDTH * FIELD_HEIGHT) as usize;
const WALL_DATA_SIZE: usize = FIELD_HEIGHT as usize;
const BOMB_DATA_SIZE: usize = FIELD_HEIGHT as usize;
const FLAG_DATA_SIZE: usize = (NUM_TILES + 1) / 2;
const VISIBILITY_DATA_SIZE: usize = FIELD_HEIGHT as usize;

pub struct Field
{
	wall_data: [u8; WALL_DATA_SIZE],
	bomb_data: [u8; BOMB_DATA_SIZE],
	flag_data: [u8; FLAG_DATA_SIZE],
}

pub struct FieldWork
{
	visibility_data: [u8; VISIBILITY_DATA_SIZE],
}

impl Field
{
	pub fn has_wall_at_rc(&self, r: u8, c: u8) -> bool
	{
		(self.wall_data[r as usize] >> (FIELD_WIDTH - 1 - c)) & 0b1 == 0b1
	}

	pub fn has_bomb_at_rc(&self, r: u8, c: u8) -> bool
	{
		(self.bomb_data[r as usize] >> (FIELD_WIDTH - 1 - c)) & 0b1 == 0b1
	}

	pub fn flag_count_from_rc(&self, r: u8, c: u8) -> u8
	{
		let data_offset = r * FIELD_WIDTH + c;
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

impl FieldWork
{
	pub fn new() -> Self
	{
		Self {
			visibility_data: [0u8; VISIBILITY_DATA_SIZE],
		}
	}

	pub fn is_visible_at_rc(&self, r: u8, c: u8) -> bool
	{
		(self.visibility_data[r as usize] >> (FIELD_WIDTH - 1 - c)) & 0b1 == 0b1
	}

	pub fn activate(&mut self, r: u8, c: u8)
	{
		self.visibility_data[r as usize] |= 0b1 << (FIELD_WIDTH - 1 - c);
	}
}

pub const NUM_FIELDS: usize = 4;
pub const FIELDS: [Field; NUM_FIELDS] = [
	Field {
		wall_data: FIELD0_WALL_DATA,
		bomb_data: FIELD0_BOMB_DATA,
		flag_data: generate_flag_data(&FIELD0_BOMB_DATA),
	},
	Field {
		wall_data: FIELD1_WALL_DATA,
		bomb_data: FIELD1_BOMB_DATA,
		flag_data: generate_flag_data(&FIELD1_BOMB_DATA),
	},
	Field {
		wall_data: FIELD2_WALL_DATA,
		bomb_data: FIELD2_BOMB_DATA,
		flag_data: generate_flag_data(&FIELD2_BOMB_DATA),
	},
	Field {
		wall_data: FIELD3_WALL_DATA,
		bomb_data: FIELD3_BOMB_DATA,
		flag_data: generate_flag_data(&FIELD3_BOMB_DATA),
	},
];

const fn generate_flag_data(
	bomb_data: &[u8; BOMB_DATA_SIZE],
) -> [u8; FLAG_DATA_SIZE]
{
	let mut flag_data = [0u8; FLAG_DATA_SIZE];
	let mut r = 0;
	while r < FIELD_HEIGHT
	{
		let mut c = 0;
		while c < FIELD_WIDTH
		{
			let flag_count_top = if r > 0
			{
				let byte = bomb_data[(r - 1) as usize] as u16;
				let bits = (((byte << 2) >> (FIELD_WIDTH - c)) & 0b111) as u8;
				(bits & 0b001) + ((bits & 0b010) >> 1) + ((bits & 0b100) >> 2)
			}
			else
			{
				0
			};
			let flag_count_mid = {
				let byte = bomb_data[r as usize] as u16;
				let bits = (((byte << 2) >> (FIELD_WIDTH - c)) & 0b101) as u8;
				(bits & 0b001) + ((bits & 0b100) >> 2)
			};
			let flag_count_bot = if r + 1 < FIELD_HEIGHT
			{
				let byte = bomb_data[(r + 1) as usize] as u16;
				let bits = (((byte << 2) >> (FIELD_WIDTH - c)) & 0b111) as u8;
				(bits & 0b001) + ((bits & 0b010) >> 1) + ((bits & 0b100) >> 2)
			}
			else
			{
				0
			};
			let flag_count = flag_count_top + flag_count_mid + flag_count_bot;

			let data_offset = r * FIELD_WIDTH + c;
			let byte_offset = (data_offset / 2) as usize;
			let needs_bit_shift: bool = (data_offset % 2) == 0;
			if needs_bit_shift
			{
				flag_data[byte_offset] |= flag_count << 4;
			}
			else
			{
				flag_data[byte_offset] |= flag_count;
			}
			c += 1;
		}
		r += 1;
	}
	flag_data
}

#[rustfmt::skip]
const FIELD0_WALL_DATA: [u8; WALL_DATA_SIZE] = [
	0b00000001,
	0b00000101,
	0b00000100,
	0b00000101,
	0b00000001,
];
#[rustfmt::skip]
const FIELD0_BOMB_DATA: [u8; BOMB_DATA_SIZE] = [
	0b00000000,
	0b00000000,
	0b00100000,
	0b00000000,
	0b00000000,
];

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

#[rustfmt::skip]
const FIELD2_WALL_DATA: [u8; WALL_DATA_SIZE] = [
	0b00011111,
	0b00000000,
	0b00000000,
	0b00000000,
	0b00011111,
];
#[rustfmt::skip]
const FIELD2_BOMB_DATA: [u8; BOMB_DATA_SIZE] = [
	0b00000000,
	0b00000010,
	0b00000110,
	0b00000000,
	0b00000000,
];

#[rustfmt::skip]
const FIELD3_WALL_DATA: [u8; WALL_DATA_SIZE] = [
	0b00000000,
	0b00000000,
	0b00000000,
	0b00000000,
	0b00000000,
];
#[rustfmt::skip]
const FIELD3_BOMB_DATA: [u8; BOMB_DATA_SIZE] = [
	0b00010011,
	0b00000100,
	0b00001101,
	0b00010001,
	0b00000111,
];
