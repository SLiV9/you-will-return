//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::field::*;
use crate::hero::*;
use crate::palette;
use crate::wasm4::*;

const WALL_HEIGHT: u32 = 45;
const HUD_HEIGHT: u32 = 23;

const Y_OF_FIELD: i32 = WALL_HEIGHT as i32 + 5;
const X_OF_FIELD: i32 =
	(SCREEN_SIZE as i32) / 2 - ((FIELD_SIZE * TILE_WIDTH) as i32) / 2;

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
			hero: Hero::new(),
		}
	}

	pub fn update(&mut self) -> Option<Transition>
	{
		self.ticks += 1;

		let geometry = self.determine_geometry();
		self.hero.update(&geometry);

		if self.hero.x > (SCREEN_SIZE as i32) + 5
			&& ((self.field_offset + 1) as usize) < NUM_FIELDS
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

		for r in 0..FIELD_SIZE
		{
			for c in 0..FIELD_SIZE
			{
				let xx = X_OF_FIELD + (TILE_WIDTH as i32) * (c as i32);
				let yy = Y_OF_FIELD + (TILE_HEIGHT as i32) * (r as i32);

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

		self.hero.draw();

		unsafe { *DRAW_COLORS = 0x20 };
		rect(0, 160 - HUD_HEIGHT as i32, 160, HUD_HEIGHT);

		unsafe { *DRAW_COLORS = 2 };
		text(
			format!("/EMPLOYEE/{}-{:03}/*", self.hero.code, self.hero.number),
			5,
			140,
		);
		unsafe { *DRAW_COLORS = 3 };
		text(format!("{:03}", self.hero.health), 133, 150);
	}

	fn determine_geometry(&self) -> Geometry
	{
		let yy = self.hero.y - Y_OF_FIELD;
		let xx = self.hero.x - X_OF_FIELD;
		let th = TILE_HEIGHT as i32;
		let tw = TILE_WIDTH as i32;
		let r = (yy + 100 * th) / th - 100;
		let row_of_up = (yy - 1 + 100 * th) / th - 100;
		let row_of_down = (yy + 1 + 100 * th) / th - 100;
		let c = (xx + 100 * tw) / tw - 100;
		let col_of_left = (xx - 1 + 100 * tw) / tw - 100;
		let col_of_right = (xx + 1 + 100 * tw) / tw - 100;
		let is_off = c < 0 || c >= (FIELD_SIZE as i32);
		let can_move_left = col_of_left == c
			|| col_of_left < 0
			|| col_of_left >= (FIELD_SIZE as i32)
			|| !self.field.has_wall_at_rc(r as u32, col_of_left as u32);
		let can_move_right = col_of_right == c
			|| col_of_right < 0
			|| col_of_right >= (FIELD_SIZE as i32)
			|| !self.field.has_wall_at_rc(r as u32, col_of_right as u32);
		let can_move_up = row_of_up == r
			|| (row_of_up >= 0 && is_off)
			|| (row_of_up >= 0
				&& !self.field.has_wall_at_rc(row_of_up as u32, c as u32));
		let can_move_down = ((yy + 1) % th) != 0
			|| (row_of_down < (FIELD_SIZE as i32) && is_off)
			|| (row_of_down < (FIELD_SIZE as i32)
				&& !self.field.has_wall_at_rc(row_of_down as u32, c as u32));
		Geometry {
			can_move_left,
			can_move_right,
			can_move_up,
			can_move_down,
		}
	}
}

pub enum Transition
{
	Next
	{
		field_offset: u8
	},
}
