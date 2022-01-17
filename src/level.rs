//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::communication::*;
use crate::dialog::*;
use crate::field::*;
use crate::hero::*;
use crate::palette;
use crate::sprites;
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

const CONFIDENCE_PERCENTAGE: u8 = 90;

pub struct Level
{
	field_offset: u8,
	field: &'static Field,
	communication: &'static Communication,
	dialog_tree: &'static DialogTree,
	field_work: FieldWork,
	dialog: Option<Dialog>,
	ticks: i32,
	hero: Hero,
	is_big_light_on: bool,
	is_translating: bool,
	left_door_height: u32,
	right_door_height: u32,
	first_hero_number: u8,
	max_col_reached: u8,
}

impl Level
{
	pub fn new(field_offset: u8, hero_number: u8) -> Self
	{
		assert!((field_offset as usize) < NUM_FIELDS);
		Self {
			field_offset,
			field: &FIELDS[field_offset as usize],
			communication: &COMMUNICATIONS[field_offset as usize],
			dialog_tree: &DIALOG_TREES[field_offset as usize],
			field_work: FieldWork::new(),
			dialog: None,
			ticks: 0,
			hero: Hero::new(hero_number),
			is_big_light_on: false,
			is_translating: field_offset > 0,
			left_door_height: (FIELD_HEIGHT as u32) * TILE_HEIGHT,
			right_door_height: 0,
			first_hero_number: hero_number,
			max_col_reached: 0,
		}
	}

	pub fn update(&mut self) -> Option<Transition>
	{
		self.ticks += 1;

		let gamepad = unsafe { *GAMEPAD1 };
		self.is_big_light_on =
			self.hero.is_alive() && (gamepad & BUTTON_2 != 0);
		if self.dialog.is_some() && (gamepad & BUTTON_1 != 0)
		{
			if !self.hero.is_visible()
			{
				self.hero = Hero::new(self.hero.number.wrapping_add(1));
			}
			self.dialog = None;
		}

		let geometry = self.determine_geometry();
		let is_scanning = self.is_big_light_on || self.dialog.is_some();
		self.hero.update(&geometry, is_scanning);

		if let Some(pos) = self.get_hero_position()
		{
			if self.is_translating
			{
				self.field_work.activate(pos.row, pos.col);
			}
			if self.field.has_bomb_at_rc(pos.row, pos.col)
			{
				self.hero.become_grabbed();
			}
			else if self.is_big_light_on
			{
				let strength = self.field.flag_count_from_rc(pos.row, pos.col);
				if strength > 1
				{
					let damage = strength - 1;
					if self.hero.health > damage
					{
						self.hero.health -= damage;
					}
					else
					{
						self.hero.health = 0;
						self.hero.collapse();
					}
				}
			}

			if self.hero.is_alive() && pos.col > self.max_col_reached
			{
				self.max_col_reached = pos.col;
				match pos.col
				{
					2 => self.dialog = self.dialog_tree.on_col_2,
					4 =>
					{
						self.dialog = self.dialog_tree.on_col_4;
						if self.field_offset == 0
						{
							self.is_translating = true;
						}
					}
					_ => (),
				}
			}
		}

		if !self.hero.is_visible()
		{
			if self.hero.number == self.first_hero_number
			{
				self.dialog = self.dialog_tree.on_first_death;
			}
			else
			{
				self.hero = Hero::new(self.hero.number.wrapping_add(1));
			}
		}

		if self.get_translation_progress_percentage() >= CONFIDENCE_PERCENTAGE
		{
			if self.right_door_height == 0
			{
				self.dialog = self.dialog_tree.on_confident_translation;
				self.right_door_height = 1;
			}
			else if self.right_door_height < 10
			{
				self.right_door_height += 1;
			}
			else if self.right_door_height < self.left_door_height
			{
				self.right_door_height = std::cmp::min(
					self.right_door_height + 5,
					self.left_door_height,
				);
			}
		}

		if self.hero.x > (SCREEN_SIZE as i32) + 5
			&& ((self.field_offset + 1) as usize) < NUM_FIELDS
		{
			Some(Transition::Next {
				field_offset: self.field_offset + 1,
				hero_number: self.hero.number,
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

		if self.left_door_height > 0
		{
			if self.field_offset == 0
			{
				unsafe { *DRAW_COLORS = 3 };
			}
			else
			{
				unsafe { *DRAW_COLORS = 2 };
			}
			for i in [0, 1, 3, 5]
			{
				let y = Y_OF_FIELD
					+ (FIELD_HEIGHT as i32) * (TILE_HEIGHT as i32) / 2
					- (self.left_door_height as i32) / 2
					+ i;
				let h = (self.left_door_height as i32) - 2 * i;
				if h >= 2
				{
					vline(i, y, h as u32);
				}
			}
		}
		if self.right_door_height > 0
		{
			unsafe { *DRAW_COLORS = 2 };
			for i in [0, 1, 3, 5]
			{
				let y = Y_OF_FIELD
					+ (FIELD_HEIGHT as i32) * (TILE_HEIGHT as i32) / 2
					- (self.right_door_height as i32) / 2
					+ i;
				let h = (self.right_door_height as i32) - 2 * i;
				if h >= 2
				{
					vline(160 - 1 - i, y, h as u32);
				}
			}
		}

		let num_lines: usize = self
			.communication
			.untranslated
			.iter()
			.position(|&x| x.len() == 0)
			.unwrap_or(NUM_LINES);
		if num_lines > 0
		{
			let progress = self.get_translation_progress_percentage() as usize;
			let y_start = 2 + (10 * (NUM_LINES - num_lines) / num_lines) as i32;
			let chunk_size = (CONFIDENCE_PERCENTAGE as usize) / (2 * num_lines);
			for i in 0..num_lines
			{
				let y = y_start + (10 * i) as i32;

				{
					let mut x = (SCREEN_SIZE as i32) - 10;
					unsafe { *DRAW_COLORS = 2 };
					// Untranslated text is ascii, so read bytes.
					for symbol in self.communication.untranslated[i].bytes()
					{
						if symbol != 0x20
						{
							sprites::alien_tile::draw(x, y, symbol);
						}
						x -= 6;
					}
				}

				if progress >= (num_lines + i + 1) * chunk_size
				{
					let line = self.communication.confident[i];
					unsafe { *DRAW_COLORS = 0x13 };
					text(line, 2, y);
				}
				else if progress >= (i + 1) * chunk_size
				{
					let line = self.communication.rough[i];
					unsafe { *DRAW_COLORS = 0x12 };
					text(line, 2, y);
				};
			}
		}

		let hero_position = self.get_hero_position();
		for r in 0..FIELD_HEIGHT
		{
			for c in 0..FIELD_WIDTH
			{
				let xx = X_OF_FIELD + (TILE_WIDTH as i32) * (c as i32);
				let yy = Y_OF_FIELD + (TILE_HEIGHT as i32) * (r as i32);
				let seed = 48274771u32
					.wrapping_add(self.field_offset as u32)
					.wrapping_mul(68389231)
					.wrapping_add((r * FIELD_WIDTH + c) as u32)
					.wrapping_mul(1012391339);

				if self.field.has_wall_at_rc(r, c)
				{
					unsafe { *DRAW_COLORS = 0x01 };
					rect(xx, yy, TILE_WIDTH, TILE_HEIGHT);
				}
				else if self.field_work.is_visible_at_rc(r, c)
					&& (self.is_big_light_on
						|| hero_position == Some(Position { row: r, col: c }))
				{
					let count = self.field.flag_count_from_rc(r, c);
					unsafe { *DRAW_COLORS = 0x01 };
					sprites::alien_tile::draw_tile(xx, yy, seed);
					unsafe { *DRAW_COLORS = 0x30 };
					rect(xx, yy, TILE_WIDTH, TILE_HEIGHT);
					unsafe { *DRAW_COLORS = 0x13 };
					if count > 0
					{
						text(format!("{}", count), xx + 4, yy + 4);
					}
				}
				else
				{
					unsafe { *DRAW_COLORS = 0x01 };
					sprites::alien_tile::draw_tile(xx, yy, seed);
				}
			}
		}

		self.hero.draw();

		unsafe { *DRAW_COLORS = 0x01 };
		{
			let yy = Y_OF_FIELD + (FIELD_HEIGHT as i32) * (TILE_HEIGHT as i32);
			rect(0, yy, 160, 160 - HUD_HEIGHT - (yy as u32));
		}

		if let Some(dialog) = &self.dialog
		{
			unsafe { *DRAW_COLORS = 0x22 };
			rect(0, 160 - HUD_HEIGHT as i32, 160, HUD_HEIGHT);

			unsafe { *DRAW_COLORS = 1 };
			let i = self.hero.number as usize;
			let j = self.hero.number.wrapping_add(1) as usize;
			if dialog.is_self
			{
				text(format!("{} >> {}", NAMES[i], NAMES[j]), 5, 140);
			}
			else
			{
				text(format!("{} >> {}", NAMES[j], NAMES[i]), 5, 140);
			};
			text("X", 147, 140);
			text(dialog.line, 5, 150);
		}
		else
		{
			unsafe { *DRAW_COLORS = 0x21 };
			rect(0, 160 - HUD_HEIGHT as i32, 160, HUD_HEIGHT);

			unsafe { *DRAW_COLORS = 2 };
			text(
				format!(
					"//ID/{:03}/{:/>6}/{}//",
					self.hero.number, self.hero.name, self.hero.initial
				),
				5,
				140,
			);
			unsafe { *DRAW_COLORS = 2 };
			text(format!("{:03}", self.hero.health), 133, 150);
		}
	}

	fn determine_geometry(&self) -> Geometry
	{
		let can_escape = self.right_door_height > 0;
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
			|| (col_of_right >= (FIELD_WIDTH as i32) && can_escape)
			|| col_of_right < 0
			|| (col_of_right < (FIELD_WIDTH as i32)
				&& !self.field.has_wall_at_rc(r as u8, col_of_right as u8));
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

	fn get_translation_progress_percentage(&self) -> u8
	{
		let max = self.field.num_reachable_tiles();
		let progress = self.field_work.num_activated_tiles();
		let percentage = std::cmp::min(progress * 100 / max, 100);
		percentage as u8
	}
}

pub enum Transition
{
	Next
	{
		field_offset: u8, hero_number: u8
	},
}

#[derive(Debug, PartialEq, Eq)]
struct Position
{
	row: u8,
	col: u8,
}
