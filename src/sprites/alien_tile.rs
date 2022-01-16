//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::wasm4::*;

pub fn draw_tile(x: i32, y: i32, mut seed: u32)
{
	let w = ALIEN_TILE_WIDTH as i32;
	let h = ALIEN_TILE_HEIGHT as i32;
	draw(x + 1, y + 1, seed);
	seed = seed.wrapping_mul(0x986569);
	draw(x + w, y + 1, seed);
	seed = seed.wrapping_mul(0xa2be03);
	draw(x + 1, y + h, seed);
	seed = seed.wrapping_mul(0x80a79b);
	draw(x + w, y + h, seed);
}

pub fn draw(x: i32, y: i32, seed: u32)
{
	blit(
		&ALIEN_TILES[(seed as usize) % NUM_ALIEN_TILES],
		x,
		y,
		ALIEN_TILE_WIDTH,
		ALIEN_TILE_HEIGHT,
		ALIEN_TILE_FLAGS,
	);
}

// alien_tile
const ALIEN_TILE_WIDTH: u32 = 8;
const ALIEN_TILE_HEIGHT: u32 = 8;
const ALIEN_TILE_FLAGS: u32 = BLIT_1BPP;

const NUM_ALIEN_TILES: usize = 7;
const ALIEN_TILES: [[u8; 8]; NUM_ALIEN_TILES] = [
	ALIEN_TILE1,
	ALIEN_TILE2,
	ALIEN_TILE3,
	ALIEN_TILE4,
	ALIEN_TILE5,
	ALIEN_TILE6,
	ALIEN_TILE7,
];

// alien_tile1
const ALIEN_TILE1: [u8; 8] = [0x01, 0x55, 0x55, 0x55, 0x45, 0x7d, 0x01, 0xff];

// alien_tile2
const ALIEN_TILE2: [u8; 8] = [0x01, 0x7d, 0x41, 0x5d, 0x41, 0x7d, 0x01, 0xff];

// alien_tile3
const ALIEN_TILE3: [u8; 8] = [0x01, 0x7d, 0x45, 0x55, 0x55, 0x55, 0x01, 0xff];

// alien_tile4
const ALIEN_TILE4: [u8; 8] = [0x01, 0x7d, 0x05, 0x75, 0x05, 0x7d, 0x01, 0xff];

// alien_tile5
const ALIEN_TILE5: [u8; 8] = [0x01, 0x7d, 0x45, 0x55, 0x45, 0x7d, 0x01, 0xff];

// alien_tile6
const ALIEN_TILE6: [u8; 8] = [0x01, 0x55, 0x45, 0x7d, 0x45, 0x55, 0x01, 0xff];

// alien_tile7
const ALIEN_TILE7: [u8; 8] = [0x01, 0x7d, 0x11, 0x55, 0x11, 0x7d, 0x01, 0xff];
