//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::sfx;
use crate::sprites;
use crate::wasm4::*;

pub struct Hero
{
	pub name: &'static str,
	pub initial: &'static str,
	pub number: u8,
	pub health: u8,
	pub x: i32,
	pub y: i32,
	pub is_dead: bool,
	pub num_death_ticks: u32,
	pub max_death_ticks: u32,
	move_audio_ticks: u32,
	sprite: sprites::astronaut::Animation,
}

pub struct Geometry
{
	pub can_move_left: bool,
	pub can_move_right: bool,
	pub can_move_up: bool,
	pub can_move_down: bool,
}

pub const NUM_NAMES: usize = 17;
pub const NAMES: [&'static str; NUM_NAMES] = [
	"WEIR", "ADAMS", "ZIMSKY", "PARKER", "SMITH", "MILLER", "KANE", "BLAIR",
	"NORRIS", "PRICE", "MACE", "KANEDA", "HUDSON", "LELAND", "COOPER",
	"DALLAS", "GORMAN",
];

pub const NUM_INITIALS: usize = 32;
pub const INITIALS: [&'static str; NUM_INITIALS] = [
	"I", "S", "L", "M", "C", "G", "K", "J", "R", "D", "L", "B", "P", "T", "N",
	"J", "M", "B", "H", "E", "R", "J", "T", "S", "D", "A", "F", "C", "A", "W",
	"V", "K",
];

impl Hero
{
	pub fn new(number: u8) -> Self
	{
		Self {
			name: NAMES[(number as usize) % NUM_NAMES],
			initial: INITIALS[(number as usize) % NUM_INITIALS],
			number,
			health: 100,
			x: -10,
			y: 90,
			is_dead: false,
			num_death_ticks: 0,
			max_death_ticks: 0,
			move_audio_ticks: 0,
			sprite: sprites::astronaut::Animation::new(),
		}
	}

	pub fn update(&mut self, walls: &Geometry, is_scanning: bool)
	{
		self.sprite.tick();

		let gamepad = unsafe { *GAMEPAD1 };
		let left = gamepad & BUTTON_LEFT != 0;
		let right = gamepad & BUTTON_RIGHT != 0;
		let up = gamepad & BUTTON_UP != 0;
		let down = gamepad & BUTTON_DOWN != 0;

		if self.is_dead
		{
			self.num_death_ticks += 1;

			if self.num_death_ticks == 22
			{
				sfx::snatch_pull();
			}
			else if self.num_death_ticks == 160
			{
				sfx::swallow();
			}
			else if self.num_death_ticks == 170
			{
				sfx::crunch();
			}
			else if self.num_death_ticks == 179
			{
				sfx::crunch();
			}
		}
		else if is_scanning
		{
			self.sprite.scan();
		}
		else if self.x > (SCREEN_SIZE as i32) - 10
		{
			self.sprite.run_right();
			self.x += 1;
		}
		else if self.x == 0 && (left && !right)
		{
			self.sprite.idle();
		}
		else if (left && !right) && walls.can_move_left
		{
			self.sprite.run_left();
			self.x -= 1;
		}
		else if (right && !left) && walls.can_move_right
		{
			self.sprite.run_right();
			self.x += 1;
		}
		else if (up && !down) && walls.can_move_up
		{
			self.sprite.run_up();
			self.y -= 1;
		}
		else if (down && !up) && walls.can_move_down
		{
			self.sprite.run_down();
			self.y += 1;
		}
		else if self.x < 10
		{
			self.sprite.run_right();
			self.x += 1;
		}
		else
		{
			self.sprite.idle();
		}

		if self.sprite.is_running()
		{
			self.move_audio_ticks += 1;
			if self.move_audio_ticks > 10
			{
				self.move_audio_ticks = 0;
				sfx::footstep(
					150 + 4 * (self.x % 7) as u32 + 3 * (self.y % 9) as u32,
				);
			}
		}
	}

	pub fn become_grabbed(&mut self)
	{
		self.is_dead = true;
		self.max_death_ticks = 180;
		self.sprite.become_grabbed();
		sfx::snatch_hit();
	}

	pub fn collapse(&mut self)
	{
		self.is_dead = true;
		self.max_death_ticks = 20;
		self.sprite.collapse();
	}

	pub fn is_alive(&self) -> bool
	{
		!self.is_dead
	}

	pub fn is_visible(&self) -> bool
	{
		self.sprite.is_visible()
	}

	pub fn draw(&self)
	{
		self.sprite.draw(self.x, self.y);
	}
}
