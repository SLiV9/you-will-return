//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::wasm4::*;

const SCALE_SIZE: usize = 8;
const SCALE: [i8; SCALE_SIZE] = [0, 2, 3, 5, 7, 8, 11, 12];
const RATE: usize = 14;

pub fn play_sample(t: usize)
{
	//let seed: usize = 487;
	//let seed: usize = 443;
	//let seed: usize = 389;
	let seed: usize = 449;
	let ground_freq = 264;
	let broken = false;
	play(t, seed, ground_freq, broken)
}

pub fn play(tick: usize, seed: usize, ground_freq: u32, broken: bool)
{
	if tick % RATE != 0
	{
		return;
	}

	let t = tick / RATE;
	let note = get_note_from_t(t, seed, broken);
	play_note(note, ground_freq, broken)
}

const fn get_note_from_t(t: usize, seed: usize, broken: bool) -> i8
{
	let ground = if broken { 13 } else { 12 };
	if t % ground == 0
	{
		0
	}
	else
	{
		let offset = seed.wrapping_mul(t) % 313;
		SCALE[offset % SCALE_SIZE]
	}
}

fn play_note(note: i8, ground_freq: u32, broken: bool)
{
	let power = if broken { 13 } else { 12 };
	let magic = 2f64.powf(1.0 / (power as f64));
	let freq: f64 = (ground_freq as f64) * magic.powf(note as f64);
	let sustain = (RATE / 2) as u32;
	let release = (RATE / 2) as u32;
	tone(
		freq.round() as u32,
		sustain | (release << 8),
		20,
		TONE_PULSE1,
	);
}
