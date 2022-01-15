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

const PROXIMITY_LIGHT_WIDTH: u32 = TILE_WIDTH * 3 / 2;
const PROXIMITY_LIGHT_HEIGHT: u32 = TILE_HEIGHT * 3 / 2;

const Y_OF_FIELD: i32 = WALL_HEIGHT as i32 + 5;
const X_OF_FIELD: i32 =
	(SCREEN_SIZE as i32) / 2 - ((FIELD_SIZE * TILE_WIDTH) as i32) / 2;

pub struct Level
{
	field_offset: u8,
	field: &'static Field,
	field_work: FieldWork,
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
			field_work: FieldWork::new(),
			ticks: 0,
			hero: Hero::new(),
		}
	}

	pub fn update(&mut self) -> Option<Transition>
	{
		self.ticks += 1;

		let geometry = self.determine_geometry();
		self.hero.update(&geometry);

		if self.hero.is_alive()
		{
			self.detect_activation();
			if self.is_hero_on_bomb()
			{
				self.hero.kill();
			}
		}
		else if !self.hero.is_visible()
		{
			self.hero = Hero::new();
		}

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
		if self.hero.x > 0 && self.hero.x < SCREEN_SIZE as i32
		{
			unsafe { *DRAW_COLORS = 0x22 };
			oval(
				self.hero.x - (PROXIMITY_LIGHT_WIDTH as i32) / 2,
				self.hero.y - (PROXIMITY_LIGHT_HEIGHT as i32) / 2,
				PROXIMITY_LIGHT_WIDTH,
				PROXIMITY_LIGHT_HEIGHT,
			);
		}

		unsafe { *DRAW_COLORS = 1 };
		rect(0, 0, X_OF_FIELD as u32, 160);
		rect(160 - X_OF_FIELD, 0, X_OF_FIELD as u32, 160);
		rect(0, 0, 160, WALL_HEIGHT);
		rect(0, WALL_HEIGHT as i32, 160, Y_OF_FIELD as u32 - WALL_HEIGHT);

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
				unsafe { *DRAW_COLORS = 0x01 };
				rect(xx + 1, yy + 1, TILE_WIDTH - 2, TILE_HEIGHT - 2);

				if self.field.has_wall_at_rc(r, c)
				{
					unsafe { *DRAW_COLORS = 0x02 };
					rect(xx, yy, TILE_WIDTH, TILE_HEIGHT);
				}
				else if self.field.has_bomb_at_rc(r, c)
					&& self.field_work.is_visible_at_rc(r, c)
				{
					unsafe { *DRAW_COLORS = 0x04 };
					rect(xx, yy, TILE_WIDTH, TILE_HEIGHT);
				}
				else if self.field_work.is_visible_at_rc(r, c)
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

		unsafe { *DRAW_COLORS = 0x01 };
		{
			let yy = Y_OF_FIELD + (FIELD_SIZE * TILE_HEIGHT) as i32;
			rect(0, yy, 160, 160 - HUD_HEIGHT - (yy as u32));
		}
		unsafe { *DRAW_COLORS = 0x31 };
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

	fn is_hero_on_bomb(&self) -> bool
	{
		let yy = self.hero.y - Y_OF_FIELD;
		let xx = self.hero.x - X_OF_FIELD;
		let th = TILE_HEIGHT as i32;
		let tw = TILE_WIDTH as i32;
		let r = (yy + 100 * th) / th - 100;
		let c = (xx + 100 * tw) / tw - 100;
		r >= 0
			&& r < (FIELD_SIZE as i32)
			&& c >= 0 && c < (FIELD_SIZE as i32)
			&& self.field.has_bomb_at_rc(r as u32, c as u32)
	}

	fn detect_activation(&mut self)
	{
		let yy = self.hero.y - Y_OF_FIELD;
		let xx = self.hero.x - X_OF_FIELD;
		let th = TILE_HEIGHT as i32;
		let tw = TILE_WIDTH as i32;
		let r = (yy + 100 * th) / th - 100;
		let c = (xx + 100 * tw) / tw - 100;
		if r >= 0
			&& r < (FIELD_SIZE as i32)
			&& c >= 0 && c < (FIELD_SIZE as i32)
		{
			self.field_work.activate(r as u32, c as u32);
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
