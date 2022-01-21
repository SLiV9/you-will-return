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
	let seed: usize = 174;
	//let seed: usize = 130;
	//let seed: usize = 76;
	//let seed: usize = 136;
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
	trace(format!("{}", note));
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

#[cfg(test)]
mod tests
{
	use super::*;

	const MAX_LOOP_LENGTH: usize = 313 * 12;

	#[test]
	fn print_loop_length()
	{
		for seed in 1usize..313usize
		{
			let mut first_notes = [0i8; MAX_LOOP_LENGTH];
			for t in 0..MAX_LOOP_LENGTH
			{
				first_notes[t] =
					((seed.wrapping_mul(t) % 313) % SCALE_SIZE) as i8
			}
			for offset in 1..=313
			{
				let second_notes = (0..MAX_LOOP_LENGTH).map(|t| {
					((seed.wrapping_mul(t + offset) % 313) % SCALE_SIZE) as i8
				});
				if first_notes.iter().zip(second_notes).all(|(a, b)| *a == b)
				{
					if offset < 313
					{
						println!("{} loops every {} frames", seed, offset);
					}
					else
					{
						println!("{} achieves maximal loopage!", seed);
					}
					break;
				}
			}
		}
	}
}
