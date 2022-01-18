//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::wasm4::*;

pub fn text_beep()
{
	tone(390, (2 << 24) | (8 << 8), 33, TONE_TRIANGLE | TONE_MODE2);
}

pub fn door_jolt()
{
	tone(190 | (180 << 16), 10 | (35 << 8), 100, TONE_NOISE);
}

pub fn door_opening()
{
	tone(180 | (190 << 16), (24 << 16) | 100, 30, TONE_NOISE);
}

pub fn door_opened()
{
	tone(150, 15 | (10 << 8), 100, TONE_NOISE);
}

pub fn door_echo(volume: u32)
{
	tone(110 + 40 * volume / 100, 20 << 8, volume / 2, TONE_NOISE);
}

pub fn interference(volume: u32)
{
	tone(230, 11, volume, TONE_NOISE);
}

pub fn heart_monitor()
{
	tone(670, 4 | (4 << 8), 4, TONE_PULSE1 | TONE_MODE2);
}

pub fn migraine()
{
	tone(
		670,
		(2 << 24) | (2 << 16) | 4 | (4 << 8),
		8,
		TONE_PULSE1 | TONE_MODE2,
	);
}

pub fn flatline()
{
	tone(
		670,
		(2 << 24) | (2 << 16) | 100 | (4 << 8),
		16,
		TONE_PULSE1 | TONE_MODE2,
	);
}
