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
use crate::sfx;
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
const H_OF_FIELD: u32 = (FIELD_HEIGHT as u32) * (TILE_HEIGHT as u32);

const CONFIDENCE_PERCENTAGE: u8 = 100;
const NORMAL_HEART_RATE_IN_TICKS: u8 = 80;
const TERRIFIED_HEART_RATE_IN_TICKS: u8 = 12;
const FATAL_HEART_RATE_IN_TICKS: u8 = 10;

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
	last_translation_update: u8,
	left_door_height: u32,
	right_door_height: u32,
	first_hero_number: u8,
	max_col_reached: u8,
	respawn_ticks: u8,
	dialog_ticks: u8,
	heart_ticks: u8,
	current_heart_rate_in_ticks: u8,
	target_heart_rate_in_ticks: u8,
	seconds_since_last_translation_update: u8,
	signal_percentage: u8,
}

impl Level
{
	pub fn new(transition: Transition) -> Self
	{
		let field_offset = transition.field_offset;
		let is_translating = field_offset > 0;

		let mut hero = Hero::new(transition.hero_number);
		if let Some(health) = transition.hero_health
		{
			hero.health = health;
		}

		Self {
			field_offset,
			field: &FIELDS[field_offset as usize],
			communication: &COMMUNICATIONS[field_offset as usize],
			dialog_tree: &DIALOG_TREES[field_offset as usize],
			field_work: FieldWork::new(),
			dialog: None,
			ticks: 0,
			hero,
			is_big_light_on: false,
			is_translating,
			last_translation_update: 0,
			left_door_height: (FIELD_HEIGHT as u32) * TILE_HEIGHT,
			right_door_height: 0,
			first_hero_number: transition.hero_number,
			max_col_reached: 0,
			respawn_ticks: 0,
			dialog_ticks: 0,
			heart_ticks: 0,
			current_heart_rate_in_ticks: NORMAL_HEART_RATE_IN_TICKS,
			target_heart_rate_in_ticks: NORMAL_HEART_RATE_IN_TICKS,
			seconds_since_last_translation_update: 0,
			signal_percentage: 99,
		}
	}

	pub fn update(&mut self) -> Option<Transition>
	{
		self.ticks += 1;

		if self.ticks % 60 == 0
		{
			if self.seconds_since_last_translation_update < 255
			{
				self.seconds_since_last_translation_update += 1;
			}
		}

		if let Some(dialog) = &self.dialog
		{
			if self.dialog_ticks < 100
			{
				self.dialog_ticks += 1;
				if self.dialog_ticks == 25
				{
					sfx::text_beep();
				}
				else if self.dialog_ticks == 30 && !dialog.is_self
				{
					sfx::text_beep();
				}
				else if self.dialog_ticks == 35 && !dialog.is_self
				{
					sfx::text_beep();
				}
				else if self.dialog_ticks == 40 && dialog.is_self
				{
					sfx::text_beep();
				}
			}
		}

		let gamepad = unsafe { *GAMEPAD1 };
		self.is_big_light_on =
			self.hero.is_alive() && (gamepad & BUTTON_2 != 0);
		if self.dialog.is_some() && (gamepad & BUTTON_1 != 0)
		{
			self.dialog = None;
			if !self.hero.is_visible()
			{
				self.hero = Hero::new(self.hero.number.wrapping_add(1));
				self.heart_ticks = 0;
				self.current_heart_rate_in_ticks = NORMAL_HEART_RATE_IN_TICKS;
				self.target_heart_rate_in_ticks = NORMAL_HEART_RATE_IN_TICKS;
				self.signal_percentage = 99;
			}
		}

		let geometry = self.determine_geometry();
		let is_looking_down = self.is_big_light_on || self.dialog.is_some();
		self.hero.update(&geometry, is_looking_down);

		if let Some(pos) = self.get_hero_position()
		{
			if self.field.has_bomb_at_rc(pos.row, pos.col)
			{
				self.hero.become_grabbed();
				self.target_heart_rate_in_ticks = TERRIFIED_HEART_RATE_IN_TICKS;
			}
			else
			{
				if self.is_translating
				{
					self.field_work.activate(pos.row, pos.col);
				}

				let strength = self.field.flag_count_from_rc(pos.row, pos.col);
				if strength > 0
					&& ((self.ticks % 10) == 0
						|| self.target_heart_rate_in_ticks
							> FATAL_HEART_RATE_IN_TICKS)
				{
					self.signal_percentage = if strength < 3
					{
						100 - strength * 40
							+ ((7 * (self.ticks / 30)) % 23) as u8
					}
					else
					{
						((7 * (self.ticks / 30)) % 13) as u8
					};

					if strength > 1 && self.is_big_light_on
					{
						sfx::interference(50);
						let damage = 3 * (strength - 1) + strength - 2;
						if self.hero.health > damage
						{
							self.hero.health -= damage;
							sfx::migraine();
							self.current_heart_rate_in_ticks =
								FATAL_HEART_RATE_IN_TICKS;
							self.target_heart_rate_in_ticks =
								FATAL_HEART_RATE_IN_TICKS;
						}
						else
						{
							self.hero.health = 0;
							self.hero.collapse();
							sfx::flatline();
							self.respawn_ticks = 60;
						}
					}
					else if strength > 1
					{
						sfx::interference(4 + 4 * strength as u32);
						self.target_heart_rate_in_ticks = 60 / strength;
					}
					else
					{
						sfx::interference(4);
						self.target_heart_rate_in_ticks = 50;
					}
				}
				else if strength == 0
				{
					self.target_heart_rate_in_ticks =
						NORMAL_HEART_RATE_IN_TICKS;
					self.signal_percentage =
						95 + ((2 * (self.ticks / 30)) % 5) as u8;
				}
			}

			if self.hero.is_alive() && pos.col > self.max_col_reached
			{
				self.max_col_reached = pos.col;
				match pos.col
				{
					2 =>
					{
						self.dialog = self.dialog_tree.on_col_2;
						self.dialog_ticks = 0;
					}
					4 =>
					{
						self.dialog = self.dialog_tree.on_col_4;
						self.dialog_ticks = 0;
						if self.field_offset == 0
						{
							self.is_translating = true;
						}
					}
					_ => (),
				}
			}
		}

		if self.hero.num_death_ticks == 175
		{
			self.hero.health = 0;
			sfx::flatline_quiet();
			self.respawn_ticks = 60;
		}
		else if self.hero.num_death_ticks >= 170
		{
			if self.hero.health > 19
			{
				self.hero.health -= 19;
			}
		}

		if self.hero.health == 0
		{
			self.current_heart_rate_in_ticks = 0;
			self.target_heart_rate_in_ticks = 0;
		}

		if self.current_heart_rate_in_ticks > 0
		{
			if self.heart_ticks > 0
			{
				self.heart_ticks -= 1;
			}
			else
			{
				self.current_heart_rate_in_ticks = ((3
					+ self.current_heart_rate_in_ticks as u32
					+ (3 * self.target_heart_rate_in_ticks as u32))
					/ 4) as u8;
				self.heart_ticks = self.current_heart_rate_in_ticks;
				if (self.current_heart_rate_in_ticks
					< NORMAL_HEART_RATE_IN_TICKS
					|| self.hero.health < 100)
					&& self.current_heart_rate_in_ticks
						> FATAL_HEART_RATE_IN_TICKS
				{
					sfx::heart_monitor();
				}
			}
		}

		if !self.hero.is_visible()
		{
			if self.hero.number == self.first_hero_number
				&& self.dialog.is_none()
			{
				self.dialog = self.dialog_tree.on_first_death;
				self.dialog_ticks = 0;
			}

			if self.respawn_ticks == 0
			{
				self.hero = Hero::new(self.hero.number.wrapping_add(1));
				self.heart_ticks = 0;
				self.current_heart_rate_in_ticks = NORMAL_HEART_RATE_IN_TICKS;
				self.target_heart_rate_in_ticks = NORMAL_HEART_RATE_IN_TICKS;
				self.signal_percentage = 99;
			}
			else if self.dialog.is_none()
			{
				self.respawn_ticks -= 1;
			}
		}

		let translation_percentage = self.get_translation_progress_percentage();
		if translation_percentage > self.last_translation_update
		{
			sfx::translation_update(translation_percentage);
			self.last_translation_update = translation_percentage;
			self.seconds_since_last_translation_update = 0;
		}

		if translation_percentage >= CONFIDENCE_PERCENTAGE
		{
			if self.right_door_height == 0
			{
				self.dialog = self.dialog_tree.on_confident_translation;
				self.dialog_ticks = 0;
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
			Some(Transition {
				field_offset: self.field_offset + 1,
				hero_number: self.hero.number,
				hero_health: Some(self.hero.health),
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

		let hero_position = self.get_hero_position();
		let scan_interference = if let Some(pos) = &hero_position
		{
			self.field.flag_count_from_rc(pos.row, pos.col)
		}
		else
		{
			0
		};
		let heavy_scan_interference =
			self.target_heart_rate_in_ticks == FATAL_HEART_RATE_IN_TICKS;

		if self.hero.x > 0 && self.hero.x < SCREEN_SIZE as i32
		{
			let max = std::cmp::min(110, self.hero.max_death_ticks);
			unsafe { *DRAW_COLORS = 0x22 };
			if self.is_big_light_on && !heavy_scan_interference
			{
				rect(0, Y_OF_FIELD, SCREEN_SIZE, H_OF_FIELD);
			}
			else if self.hero.num_death_ticks == 0
			{
				let percentage = if heavy_scan_interference
				{
					50
				}
				else
				{
					100 - 10 * (scan_interference as u32)
				};
				let w = PROXIMITY_LIGHT_WIDTH * percentage / 100;
				let h = PROXIMITY_LIGHT_HEIGHT * percentage / 100;
				let x = self.hero.x - (w as i32) / 2;
				let y = self.hero.y - (h as i32) / 2;
				oval(x, y, w, h);
			}
			else if self.hero.num_death_ticks < max
			{
				let strength = max - self.hero.num_death_ticks;
				let w = PROXIMITY_LIGHT_WIDTH * strength / max;
				let h = PROXIMITY_LIGHT_HEIGHT * strength / max;
				let x = self.hero.x - (w as i32) / 2;
				let y = self.hero.y - (h as i32) / 2;
				oval(x, y, w, h);
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
				let y = Y_OF_FIELD + (H_OF_FIELD as i32) / 2
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
				let y = Y_OF_FIELD + (H_OF_FIELD as i32) / 2
					- (self.right_door_height as i32) / 2
					+ i;
				let h = (self.right_door_height as i32) - 2 * i;
				if h >= 2
				{
					vline(160 - 1 - i, y, h as u32);
				}
			}
		}

		let progress = self.get_translation_progress_percentage() as usize;

		let num_lines: usize = self
			.communication
			.untranslated
			.iter()
			.position(|&x| x.len() == 0)
			.unwrap_or(NUM_LINES);
		if num_lines > 0
		{
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
				else if heavy_scan_interference
				{
					unsafe { *DRAW_COLORS = 0x01 };
					sprites::alien_tile::draw_tile(xx, yy, seed);
					if seed.wrapping_add((self.ticks as u32) / 35) % 3 == 0
					{
						unsafe { *DRAW_COLORS = 0x30 };
						rect(xx, yy, TILE_WIDTH, TILE_HEIGHT);
						let count =
							seed.wrapping_add((self.ticks as u32) / 10) % 4;
						if count > 0
						{
							unsafe { *DRAW_COLORS = 0x13 };
							text(format!("{}", count), xx + 4, yy + 4);
						}
					}
				}
				else if self.is_big_light_on
					&& scan_interference > 0
					&& self.field_work.is_visible_at_rc(r, c)
				{
					unsafe { *DRAW_COLORS = 0x01 };
					sprites::alien_tile::draw_tile(xx, yy, seed);
					unsafe { *DRAW_COLORS = 0x30 };
					rect(xx, yy, TILE_WIDTH, TILE_HEIGHT);
					if (seed.wrapping_add((self.ticks as u32) / 22)) % 3 == 0
					{
						unsafe { *DRAW_COLORS = 0x13 };
						text("1", xx + 4, yy + 4);
					}
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
			let yy = Y_OF_FIELD + (H_OF_FIELD as i32);
			rect(0, yy, 160, 160 - HUD_HEIGHT - (yy as u32));
		}

		if let Some(dialog) = &self.dialog
		{
			unsafe { *DRAW_COLORS = 0x22 };
			rect(0, 160 - HUD_HEIGHT as i32, 160, HUD_HEIGHT);

			unsafe { *DRAW_COLORS = 1 };
			let i = (self.hero.number as usize) % NUM_NAMES;
			let j = (self.hero.number.wrapping_add(1) as usize) % NUM_NAMES;
			if dialog.is_self
			{
				text(format!("{} >> {}", NAMES[i], NAMES[j]), 5, 140);
			}
			else if self.dialog_ticks > 20
			{
				text(format!("{} >> {}", NAMES[j], NAMES[i]), 5, 140);
			}
			text("X", 147, 140);
			if self.dialog_ticks > 20
			{
				text(dialog.line, 5, 150);
			}
		}
		else
		{
			unsafe { *DRAW_COLORS = 0x21 };
			rect(0, 160 - HUD_HEIGHT as i32, 160, HUD_HEIGHT);

			if self.is_translating
				&& self.seconds_since_last_translation_update > 20
				&& scan_interference == 0
			{
				unsafe { *DRAW_COLORS = 2 };
				rect(0, 160 - HUD_HEIGHT as i32, 160, 11);
				unsafe { *DRAW_COLORS = 1 };
				if self.get_translation_progress_percentage()
					>= CONFIDENCE_PERCENTAGE
				{
					text("Head to the exit!", 5, 139);
				}
				else
				{
					text("Hold Z to scan", 5, 139);
				}
			}
			else
			{
				unsafe { *DRAW_COLORS = 2 };
				text(
					format!(
						"//ID/{:03}/{:/>6}/{}//",
						self.hero.number, self.hero.name, self.hero.initial
					),
					5,
					140,
				);
			}

			if self.is_translating
			{
				let x = 5;
				let maxw = 45;
				unsafe { *DRAW_COLORS = 0x22 };
				rect(x, 150, maxw, 7);
				let w = maxw * (progress as u32) / 100;
				unsafe { *DRAW_COLORS = 0x33 };
				if w > 0
				{
					rect(x, 150, w, 7);
				}
				unsafe { *DRAW_COLORS = 0x10 };
				for i in 0..4
				{
					let seed = 48274771u32
						.wrapping_add(self.field_offset as u32)
						.wrapping_mul(68389231);
					let xx = x + 7 + 8 * i;
					let symbol = ((seed >> (8 * i)) & 0xFF) as u8;
					sprites::alien_tile::draw(xx, 150, symbol);
				}
				unsafe { *DRAW_COLORS = 1 };
				vline(x + 6, 150, 7);
				hline(x + 1, 153, 4);
				hline(x + maxw as i32 - 5, 153, 4);
				unsafe { *DRAW_COLORS = 2 };
				hline(x, 150, maxw);
				hline(x, 156, maxw);
				unsafe { *DRAW_COLORS = 3 };
				if w > 0
				{
					hline(x, 150, w);
					hline(x, 156, w);
				}
			}
			else
			{
				text("!SYS4*", 5, 150);
			}

			unsafe { *DRAW_COLORS = 2 };
			let depth_in_px = (self.field_offset as i32) * 216 + self.hero.x;
			let snatch_h = std::cmp::min(self.hero.num_death_ticks, 150);
			let depth =
				std::cmp::max(0, depth_in_px) / 8 - (snatch_h / 10) as i32;
			text(format!("{:03}m", depth), 53, 150);

			if self.hero.is_alive() && self.signal_percentage < 90
			{
				unsafe { *DRAW_COLORS = 3 };
			}
			else
			{
				unsafe { *DRAW_COLORS = 2 };
			}
			for bar in 0..4
			{
				if self.signal_percentage >= 25 * bar
				{
					let w = (bar as u32) + 1;
					hline(87, 156 - 2 * (bar as i32), w);
				}
			}
			text(format!("{:>2}%", self.signal_percentage), 93, 150);

			if self.target_heart_rate_in_ticks <= FATAL_HEART_RATE_IN_TICKS
			{
				unsafe { *DRAW_COLORS = 3 };
			}
			else
			{
				unsafe { *DRAW_COLORS = 2 };
			}
			if self.current_heart_rate_in_ticks > 0 && self.heart_ticks > 0
			{
				let t_of_beat: i32 = 16 * (self.heart_ticks as i32)
					/ (self.current_heart_rate_in_ticks as i32);
				for t in 0..13
				{
					let (y, h) = if t + 4 == t_of_beat
					{
						(150, 5)
					}
					else if t + 5 == t_of_beat
					{
						(155, 1)
					}
					else if t + 3 == t_of_beat
					{
						(155, 2)
					}
					else
					{
						(154, 1)
					};
					vline(119 + t, y, h);
				}
			}
			else
			{
				hline(119, 154, 13);
			}
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

pub struct Transition
{
	pub field_offset: u8,
	pub hero_number: u8,
	pub hero_health: Option<u8>,
}

#[derive(Debug, PartialEq, Eq)]
struct Position
{
	row: u8,
	col: u8,
}
