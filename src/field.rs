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
	const fn generate(
		wall_data: [u8; WALL_DATA_SIZE],
		bomb_data: [u8; BOMB_DATA_SIZE],
	) -> Field
	{
		let flag_data = generate_flag_data(&bomb_data);
		Field {
			wall_data,
			bomb_data,
			flag_data,
		}
	}

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

	pub fn num_reachable_tiles(&self) -> usize
	{
		let mut count = 0;
		for r in 0..FIELD_HEIGHT
		{
			for c in 0..FIELD_WIDTH
			{
				if !self.has_wall_at_rc(r, c) && !self.has_bomb_at_rc(r, c)
				{
					count += 1;
				}
			}
		}
		count
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

	pub fn num_activated_tiles(&self) -> usize
	{
		let mut count = 0;
		for r in 0..FIELD_HEIGHT
		{
			for c in 0..FIELD_WIDTH
			{
				if self.is_visible_at_rc(r, c)
				{
					count += 1;
				}
			}
		}
		count
	}
}

pub const NUM_FIELDS: usize = 13;
pub const FIELDS: [Field; NUM_FIELDS] = [
	F_EMPTY_HALLWAY,
	F_CENTER_WALL,
	F_TWO_ROOMS,
	F_FIRST_BOMB,
	F_DOUBLE_BOMB,
	F_J_SHAPE,
	F_J_SHAPE,
	F_J_SHAPE,
	F_J_SHAPE,
	F_J_SHAPE,
	F_J_SHAPE,
	F_J_SHAPE,
	F_FINAL_ROOM,
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

pub const F_TEST: Field = {
	#[rustfmt::skip]
	const WALL_DATA: [u8; WALL_DATA_SIZE] = [
		0b00000001,
		0b00000001,
		0b00000000,
		0b00000101,
		0b00000001,
	];
	#[rustfmt::skip]
	const BOMB_DATA: [u8; BOMB_DATA_SIZE] = [
		0b00000100,
		0b00011100,
		0b00110100,
		0b00000000,
		0b00000000,
	];
	Field::generate(WALL_DATA, BOMB_DATA)
};

const F_EMPTY_HALLWAY: Field = {
	#[rustfmt::skip]
	const WALL_DATA: [u8; WALL_DATA_SIZE] = [
		0b11000011,
		0b00000000,
		0b00000000,
		0b00000000,
		0b11000011,
	];
	#[rustfmt::skip]
	const BOMB_DATA: [u8; BOMB_DATA_SIZE] = [
		0b00000000,
		0b00000000,
		0b00000000,
		0b00000000,
		0b00000000,
	];
	Field::generate(WALL_DATA, BOMB_DATA)
};

const F_CENTER_WALL: Field = {
	#[rustfmt::skip]
	const WALL_DATA: [u8; WALL_DATA_SIZE] = [
		0b00000000,
		0b00100100,
		0b00100100,
		0b00100100,
		0b00000000,
	];
	#[rustfmt::skip]
	const BOMB_DATA: [u8; BOMB_DATA_SIZE] = [
		0b00000000,
		0b00000000,
		0b00000000,
		0b00000000,
		0b00000000,
	];
	Field::generate(WALL_DATA, BOMB_DATA)
};

const F_TWO_ROOMS: Field = {
	#[rustfmt::skip]
	const WALL_DATA: [u8; WALL_DATA_SIZE] = [
		0b10000010,
		0b11101110,
		0b00000000,
		0b11101110,
		0b10000010,
	];
	#[rustfmt::skip]
	const BOMB_DATA: [u8; BOMB_DATA_SIZE] = [
		0b00000000,
		0b00000000,
		0b00000000,
		0b00000000,
		0b00000000,
	];
	Field::generate(WALL_DATA, BOMB_DATA)
};

const F_FIRST_BOMB: Field = {
	#[rustfmt::skip]
	const WALL_DATA: [u8; WALL_DATA_SIZE] = [
		0b11111111,
		0b00000000,
		0b00000000,
		0b00000000,
		0b11111111,
	];
	#[rustfmt::skip]
	const BOMB_DATA: [u8; BOMB_DATA_SIZE] = [
		0b00000000,
		0b00000000,
		0b00000100,
		0b00000000,
		0b00000000,
	];
	Field::generate(WALL_DATA, BOMB_DATA)
};

const F_DOUBLE_BOMB: Field = {
	#[rustfmt::skip]
	const WALL_DATA: [u8; WALL_DATA_SIZE] = [
		0b11001001,
		0b01101001,
		0b00000000,
		0b00100001,
		0b11111111,
	];
	#[rustfmt::skip]
	const BOMB_DATA: [u8; BOMB_DATA_SIZE] = [
		0b00000000,
		0b00000100,
		0b00000000,
		0b00010000,
		0b00000000,
	];
	Field::generate(WALL_DATA, BOMB_DATA)
};

const F_J_SHAPE: Field = {
	#[rustfmt::skip]
	const WALL_DATA: [u8; WALL_DATA_SIZE] = [
		0b11111000,
		0b00000000,
		0b01000000,
		0b01000011,
		0b11111111,
	];
	#[rustfmt::skip]
	const BOMB_DATA: [u8; BOMB_DATA_SIZE] = [
		0b00000000,
		0b00001000,
		0b00011000,
		0b00000000,
		0b00000000,
	];
	Field::generate(WALL_DATA, BOMB_DATA)
};

const F_DIAGONAL: Field = {
	#[rustfmt::skip]
	const WALL_DATA: [u8; WALL_DATA_SIZE] = [
		0b00000000,
		0b00000000,
		0b00000000,
		0b00000000,
		0b00000000,
	];
	#[rustfmt::skip]
	const BOMB_DATA: [u8; BOMB_DATA_SIZE] = [
		0b00010011,
		0b00000100,
		0b00001101,
		0b00010001,
		0b00000111,
	];
	Field::generate(WALL_DATA, BOMB_DATA)
};

const F_FINAL_ROOM: Field = {
	#[rustfmt::skip]
	const WALL_DATA: [u8; WALL_DATA_SIZE] = [
		0b00011111,
		0b00001110,
		0b00000000,
		0b00001110,
		0b00011111,
	];
	#[rustfmt::skip]
	const BOMB_DATA: [u8; BOMB_DATA_SIZE] = [
		0b11100000,
		0b10110001,
		0b11111111,
		0b10110001,
		0b11100000,
	];
	Field::generate(WALL_DATA, BOMB_DATA)
};
