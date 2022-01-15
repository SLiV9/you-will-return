//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::palette;
use crate::wasm4::*;

use crate::field::*;

const WALL_HEIGHT: u32 = 45;
const HUD_HEIGHT: u32 = 23;

pub struct Level
{
	field_offset: u8,
	field: &'static Field,
	ticks: i32,
	hero: Hero,
}

impl Level
{
	pub fn new(field_offset: u8) -> Self
	{
		assert!((field_offset as usize) < NUM_FIELDS);
		Self {
			field_offset,
			field: &FIELDS[field_offset as usize],
			ticks: 0,
			hero: Hero {
				code: "EKA".to_string(),
				number: 1,
				health: 100,
			},
		}
	}

	pub fn update(&mut self) -> Option<Transition>
	{
		self.ticks += 1;

		if self.ticks > 120 && ((self.field_offset + 1) as usize) < NUM_FIELDS
		{
			Some(Transition::Next {
				field_offset: self.field_offset + 1,
			})
		}
		else
		{
			None
		}
	}

	pub fn draw(&mut self)
	{
		unsafe { *DRAW_COLORS = 2 };
		rect(0, 0, 160, WALL_HEIGHT);

		unsafe { *DRAW_COLORS = 3 };
		text("HERE IS A GIFT", 10, 4);
		text("WE MADE THIS GIFT", 10, 14);
		text("FOR YOU", 10, 24);

		let y_of_field = WALL_HEIGHT as i32 + 5;
		let x_of_field =
			(SCREEN_SIZE as i32) / 2 - ((FIELD_SIZE * TILE_WIDTH) as i32) / 2;
		for r in 0..FIELD_SIZE
		{
			for c in 0..FIELD_SIZE
			{
				let xx = x_of_field + (TILE_WIDTH as i32) * (c as i32);
				let yy = y_of_field + (TILE_HEIGHT as i32) * (r as i32);

				if self.field.has_wall_at_rc(r, c)
				{
					unsafe { *DRAW_COLORS = 0x02 };
					rect(xx, yy, TILE_WIDTH, TILE_HEIGHT);
				}
				else if self.field.has_bomb_at_rc(r, c)
				{
					unsafe { *DRAW_COLORS = 0x04 };
					rect(xx, yy, TILE_WIDTH, TILE_HEIGHT);
				}
				else
				{
					let count = self.field.flag_count_from_rc(r, c);
					unsafe { *DRAW_COLORS = 0x30 };
					rect(xx, yy, TILE_WIDTH, TILE_HEIGHT);
					unsafe { *DRAW_COLORS = 0x4 };
					if count > 0
					{
						text(format!("{}", count), xx + 3, yy + 3);
					}
				}
			}
		}

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

pub enum Transition
{
	Next
	{
		field_offset: u8
	},
}

struct Hero
{
	code: String,
	number: i32,
	health: i32,
}
