//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::wasm4::*;

pub fn draw(x: i32, y: i32)
{
	blit(
		&ROCKWALL_TOP_ONLY,
		x,
		y,
		ROCKWALL_TOP_ONLY_WIDTH,
		ROCKWALL_TOP_ONLY_HEIGHT,
		ROCKWALL_TOP_ONLY_FLAGS,
	);
}

// rockwall_top_only
const ROCKWALL_TOP_ONLY_WIDTH: u32 = 16;
const ROCKWALL_TOP_ONLY_HEIGHT: u32 = 16;
const ROCKWALL_TOP_ONLY_FLAGS: u32 = 0; // BLIT_1BPP
const ROCKWALL_TOP_ONLY: [u8; 32] = [
	0x73, 0x86, 0xff, 0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
	0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];
