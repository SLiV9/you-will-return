//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::sprites;
use crate::wasm4::*;

pub const MAX_DEATH_TICKS: u32 = 150;

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
	}

	pub fn become_grabbed(&mut self)
	{
		self.is_dead = true;
		self.sprite.become_grabbed();
	}

	pub fn collapse(&mut self)
	{
		self.is_dead = true;
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
