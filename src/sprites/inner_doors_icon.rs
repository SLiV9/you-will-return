//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::wasm4::*;

pub fn draw(x: i32, y: i32)
{
	blit(
		&INNER_DOORS_ICON,
		x - (INNER_DOORS_ICON_WIDTH as i32) / 2,
		y - (INNER_DOORS_ICON_HEIGHT as i32) + 1,
		INNER_DOORS_ICON_WIDTH,
		INNER_DOORS_ICON_HEIGHT,
		INNER_DOORS_ICON_FLAGS,
	);
}

// inner_doors_icon
const INNER_DOORS_ICON_WIDTH: u32 = 8;
const INNER_DOORS_ICON_HEIGHT: u32 = 8;
const INNER_DOORS_ICON_FLAGS: u32 = BLIT_1BPP;
const INNER_DOORS_ICON: [u8; 8] =
	[0x00, 0x00, 0x00, 0x36, 0x77, 0x77, 0x77, 0x77];
