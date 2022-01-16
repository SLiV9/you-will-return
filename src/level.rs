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

const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;

const PROXIMITY_LIGHT_WIDTH: u32 = TILE_WIDTH * 3 / 2;
const PROXIMITY_LIGHT_HEIGHT: u32 = TILE_HEIGHT * 3 / 2;

const Y_OF_FIELD: i32 = WALL_HEIGHT as i32 + 5;
const X_OF_FIELD: i32 =
	(SCREEN_SIZE as i32) / 2 - (((FIELD_WIDTH as u32) * TILE_WIDTH) as i32) / 2;

pub struct Level
{
	field_offset: u8,
	field: &'static Field,
	field_work: FieldWork,
	ticks: i32,
	hero: Hero,
	is_big_light_on: bool,
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
			is_big_light_on: false,
		}
	}

	pub fn update(&mut self) -> Option<Transition>
	{
		self.ticks += 1;

		self.is_big_light_on = if self.hero.is_alive()
		{
			let gamepad = unsafe { *GAMEPAD1 };
			gamepad & BUTTON_1 != 0
		}
		else
		{
			false
		};

		let geometry = self.determine_geometry();
		self.hero.update(&geometry, self.is_big_light_on);

		if let Some(pos) = self.get_hero_position()
		{
			self.field_work.activate(pos.row, pos.col);
			if self.field.has_bomb_at_rc(pos.row, pos.col)
			{
				self.hero.become_grabbed();
			}
			else if self.is_big_light_on
			{
				let damage = self.field.flag_count_from_rc(pos.row, pos.col);
				self.hero.health -= damage as i32;
				if self.hero.health <= 0
				{
					self.hero.health = 0;
					self.hero.collapse();
				}
			}
		}

		if !self.hero.is_visible()
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
		unsafe { *PALETTE = palette::LEVEL };

		if self.hero.x > 0 && self.hero.x < SCREEN_SIZE as i32
		{
			unsafe { *DRAW_COLORS = 0x22 };
			if self.is_big_light_on
			{
				rect(0, 0, SCREEN_SIZE, SCREEN_SIZE);
			}
			else
			{
				let max = crate::hero::MAX_DEATH_TICKS;
				if self.hero.num_death_ticks < max
				{
					let strength = max - self.hero.num_death_ticks;
					let w = PROXIMITY_LIGHT_WIDTH * strength / max;
					let h = PROXIMITY_LIGHT_HEIGHT * strength / max;
					oval(
						self.hero.x - (w as i32) / 2,
						self.hero.y - (h as i32) / 2,
						w,
						h,
					);
				}
			}
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

		let hero_position = self.get_hero_position();
		for r in 0..FIELD_HEIGHT
		{
			for c in 0..FIELD_WIDTH
			{
				let xx = X_OF_FIELD + (TILE_WIDTH as i32) * (c as i32);
				let yy = Y_OF_FIELD + (TILE_HEIGHT as i32) * (r as i32);
				unsafe { *DRAW_COLORS = 0x01 };
				rect(xx + 1, yy + 1, TILE_WIDTH - 2, TILE_HEIGHT - 2);

				if self.field.has_wall_at_rc(r, c)
				{
					unsafe { *DRAW_COLORS = 0x01 };
					rect(xx, yy, TILE_WIDTH, TILE_HEIGHT);
				}
				else if self.field.has_bomb_at_rc(r, c)
					&& self.field_work.is_visible_at_rc(r, c)
				{
					unsafe { *DRAW_COLORS = 0x04 };
					rect(xx, yy, TILE_WIDTH, TILE_HEIGHT);
				}
				else if self.field_work.is_visible_at_rc(r, c)
					&& (self.is_big_light_on
						|| hero_position == Some(Position { row: r, col: c }))
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
			let yy = Y_OF_FIELD + (FIELD_HEIGHT as i32) * (TILE_HEIGHT as i32);
			rect(0, yy, 160, 160 - HUD_HEIGHT - (yy as u32));
		}
		unsafe { *DRAW_COLORS = 0x22 };
		rect(0, 160 - HUD_HEIGHT as i32, 160, HUD_HEIGHT);

		unsafe { *DRAW_COLORS = 1 };
		text(
			format!("/EMPLOYEE/{}-{:03}/*", self.hero.code, self.hero.number),
			5,
			140,
		);
		unsafe { *DRAW_COLORS = 1 };
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
		let is_off = c < 0 || c >= (FIELD_WIDTH as i32);
		let can_move_left = col_of_left == c
			|| col_of_left < 0
			|| col_of_left >= (FIELD_WIDTH as i32)
			|| !self.field.has_wall_at_rc(r as u8, col_of_left as u8);
		let can_move_right = col_of_right == c
			|| col_of_right < 0
			|| col_of_right >= (FIELD_WIDTH as i32)
			|| !self.field.has_wall_at_rc(r as u8, col_of_right as u8);
		let can_move_up = row_of_up == r
			|| (row_of_up >= 0 && is_off)
			|| (row_of_up >= 0
				&& !self.field.has_wall_at_rc(row_of_up as u8, c as u8));
		let can_move_down = ((yy + 1) % th) != 0
			|| (row_of_down < (FIELD_HEIGHT as i32) && is_off)
			|| (row_of_down < (FIELD_HEIGHT as i32)
				&& !self.field.has_wall_at_rc(row_of_down as u8, c as u8));
		Geometry {
			can_move_left,
			can_move_right,
			can_move_up,
			can_move_down,
		}
	}

	fn get_hero_position(&self) -> Option<Position>
	{
		if !self.hero.is_alive()
		{
			return None;
		}

		let yy = self.hero.y - Y_OF_FIELD;
		let xx = self.hero.x - X_OF_FIELD;
		let th = TILE_HEIGHT as i32;
		let tw = TILE_WIDTH as i32;
		let r = (yy + 100 * th) / th - 100;
		let c = (xx + 100 * tw) / tw - 100;
		if r >= 0
			&& r < (FIELD_HEIGHT as i32)
			&& c >= 0 && c < (FIELD_WIDTH as i32)
		{
			Some(Position {
				row: r as u8,
				col: c as u8,
			})
		}
		else
		{
			None
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

#[derive(Debug, PartialEq, Eq)]
struct Position
{
	row: u8,
	col: u8,
}
