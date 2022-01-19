//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::wasm4::*;

pub fn draw(x: i32, y: i32)
{
	blit(
		&VAULT_ICON,
		x - (VAULT_ICON_WIDTH as i32) / 2,
		y - (VAULT_ICON_HEIGHT as i32) + 1,
		VAULT_ICON_WIDTH,
		VAULT_ICON_HEIGHT,
		VAULT_ICON_FLAGS,
	);
}

// vault_icon
const VAULT_ICON_WIDTH: u32 = 8;
const VAULT_ICON_HEIGHT: u32 = 10;
const VAULT_ICON_FLAGS: u32 = BLIT_1BPP;
const VAULT_ICON: [u8; 10] =
	[0x08, 0x2c, 0x2c, 0x6e, 0xef, 0xef, 0xef, 0xef, 0xef, 0xef];
