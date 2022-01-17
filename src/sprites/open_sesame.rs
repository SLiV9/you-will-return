//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::wasm4::*;

pub fn draw(x: i32, y: i32)
{
	blit(
		&OPEN_SESAME,
		x - (OPEN_SESAME_WIDTH as i32) / 2,
		y - (OPEN_SESAME_HEIGHT as i32) + 1,
		OPEN_SESAME_WIDTH,
		OPEN_SESAME_HEIGHT,
		OPEN_SESAME_FLAGS,
	);
}

// open_sesame
const OPEN_SESAME_WIDTH: u32 = 8;
const OPEN_SESAME_HEIGHT: u32 = 10;
const OPEN_SESAME_FLAGS: u32 = 0; // BLIT_1BPP
const OPEN_SESAME: [u8; 10] =
	[0x18, 0x3c, 0x3c, 0x7e, 0xbd, 0xbd, 0x3c, 0x24, 0x24, 0x24];
