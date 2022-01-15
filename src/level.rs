//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::palette;
use crate::wasm4::*;

const WALL_HEIGHT: u32 = 45;
const HUD_HEIGHT: u32 = 23;

pub struct Level
{
	ticks: i32,
	hero: Hero,
}

impl Level
{
	pub fn new(_rng_seed: u64) -> Self
	{
		Self {
			ticks: 0,
			hero: Hero {
				code: "EKA".to_string(),
				number: 1,
				health: 100,
			},
		}
	}

	pub fn update(&mut self)
	{
		self.ticks += 1;
	}

	pub fn draw(&mut self)
	{
		unsafe { *DRAW_COLORS = 2 };
		rect(0, 0, 160, WALL_HEIGHT);

		unsafe { *DRAW_COLORS = 3 };
		text("OUR ULTIMATE GIFT", 10, 4);
		text("UNLEASH THE ENERGY", 10, 14);
		text("YOUR BREAKTHROUGH", 10, 24);
		text("INTO THE FUTURE", 10, 34);

		unsafe { *DRAW_COLORS = 4 };
		text("OUR FINAL WARNING", 10, 4);
		text("IF YOU BREACH YOU", 10, 14);
		text("UNLEASH THE DANGER", 10, 24);
		text("FOREVER", 10, 34);

		// TODO remove
		unsafe { *DRAW_COLORS = 0x30 };
		rect(0, WALL_HEIGHT as i32, 160, 90);

		unsafe { *DRAW_COLORS = 0x20 };
		rect(0, 137, 160, HUD_HEIGHT);

		unsafe { *DRAW_COLORS = 2 };
		text(
			format!("/EMPLOYEE/{}-{:03}/*", self.hero.code, self.hero.number),
			5,
			140,
		);
		unsafe { *DRAW_COLORS = 3 };
		text(format!("{:03}", self.hero.health), 133, 150);
	}
}

struct Hero
{
	code: String,
	number: i32,
	health: i32,
}
