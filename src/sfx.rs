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

pub fn the_sound()
{
	tone(170, (50 << 24) | 110 | (20 << 8), 70, TONE_NOISE);
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

pub fn flatline_quiet()
{
	tone(670, 100 | (4 << 8), 2, TONE_PULSE1 | TONE_MODE2);
}

pub fn footstep(frequency: u32)
{
	tone(frequency | (20 << 16), 4, 12, TONE_PULSE2);
}

pub fn other_footstep(frequency: u32)
{
	tone(frequency | (20 << 16), 4, 6, TONE_PULSE2 | TONE_MODE2);
}

pub fn snatch_hit()
{
	tone(20 | (40 << 16), (8 << 24) | (8 << 8), 40, TONE_PULSE2);
}

pub fn snatch_pull()
{
	tone(20, (12 << 24) | (18 << 16) | 4 | (8 << 8), 40, TONE_PULSE2);
}

pub fn crunch()
{
	tone(5 | (2 << 16), 20 << 16, 80, TONE_PULSE2);
}

pub fn swallow()
{
	tone(140 | (80 << 16), (4 << 24) | 4, 80, TONE_NOISE);
}

#[allow(unused)]
pub fn something()
{
	tone(10, (12 << 24) | (18 << 16) | 4 | 28, 40, TONE_PULSE2);
}
