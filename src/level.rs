//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::palette;
use crate::wasm4::*;

const BANNER_HEIGHT: u32 = 20;
const PADDING_SIZE: u32 = 10;

pub struct Level
{
	ticks: i32,
}

impl Level
{
	pub fn new(rng_seed: u64) -> Self
	{
		Self { ticks: 0 }
	}

	pub fn update(&mut self)
	{
		self.ticks += 1;
	}

	pub fn draw(&mut self)
	{
		// TODO
	}
}
