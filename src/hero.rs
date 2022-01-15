//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::sprites;
use crate::wasm4::*;

// TODO set to duration of death animation
pub const MAX_DEATH_TICKS: u32 = 90;

pub struct Hero
{
	pub code: &'static str,
	pub number: i32,
	pub health: i32,
	pub x: i32,
	pub y: i32,
	pub is_dead: bool,
	pub num_death_ticks: u32,
	sprite: sprites::little_guy::Animation,
}

pub struct Geometry
{
	pub can_move_left: bool,
	pub can_move_right: bool,
	pub can_move_up: bool,
	pub can_move_down: bool,
}

impl Hero
{
	pub fn new() -> Self
	{
		Self {
			code: "EKA",
			number: 1,
			health: 100,
			x: -10,
			y: 90,
			is_dead: false,
			num_death_ticks: 0,
			sprite: sprites::little_guy::Animation::new(),
		}
	}

	pub fn update(&mut self, walls: &Geometry)
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

	pub fn kill(&mut self)
	{
		self.is_dead = true;
		self.sprite.die();
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
