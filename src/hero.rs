//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::sprites;
use crate::wasm4::*;

pub struct Hero
{
	pub code: &'static str,
	pub number: i32,
	pub health: i32,
	pub x: i32,
	pub y: i32,
	pub is_dead: bool,
	sprite: sprites::little_guy::Animation,
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
			y: 80,
			is_dead: false,
			sprite: sprites::little_guy::Animation::new(),
		}
	}

	pub fn update(&mut self)
	{
		self.sprite.tick();

		let gamepad = unsafe { *GAMEPAD1 };
		let left = gamepad & BUTTON_LEFT != 0;
		let right = gamepad & BUTTON_RIGHT != 0;
		let up = gamepad & BUTTON_UP != 0;
		let down = gamepad & BUTTON_DOWN != 0;

		if self.is_dead
		{
			// Nothing
		}
		else if self.x < 7 || self.x > (SCREEN_SIZE as i32) - 7
		{
			self.sprite.run_right();
			self.x += 1;
		}
		else if left && !right
		{
			self.sprite.run_left();
			self.x -= 1;
		}
		else if right && !left
		{
			self.sprite.run_right();
			self.x += 1;
		}
		else if up && !down
		{
			self.sprite.run_up();
			self.y -= 1;
		}
		else if down && !up
		{
			self.sprite.run_down();
			self.y += 1;
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

	pub fn draw(&self)
	{
		self.sprite.draw(self.x, self.y);
	}
}
