//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::palette;
use crate::sprites::stars;
use crate::wasm4::*;

pub struct Menu
{
	ticks: u32,
	is_starting: bool,
}

impl Menu
{
	pub const fn new() -> Self
	{
		Self {
			ticks: 0,
			is_starting: false,
		}
	}

	pub fn update(&mut self) -> Option<Transition>
	{
		self.ticks += 1;

		if !self.is_starting
		{
			let gamepad = unsafe { *GAMEPAD1 };
			if gamepad & BUTTON_1 != 0
			{
				self.is_starting = true;
				self.ticks = 0;
			}
		}

		if self.is_starting && self.ticks > 30
		{
			Some(Transition::Start)
		}
		else
		{
			None
		}
	}

	pub fn draw(&mut self)
	{
		unsafe { *PALETTE = palette::MENU };

		if self.is_starting
		{
			stars::draw_bg(-12);
			stars::draw(-24);

			if self.ticks < 5
			{
				let progress = self.ticks;
				let x = -15 + 3 * (progress as i32);
				let y = 90 - 3 * (progress as i32);
				let size = 160;
				unsafe { *DRAW_COLORS = 0x22 }
				oval(x, y, size, size);
				unsafe { *DRAW_COLORS = 0x11 }
				let xx = ((size as i32) + x) as u32;
				let yy = SCREEN_SIZE - 40 - 25 * progress;
				if xx < SCREEN_SIZE
				{
					rect(
						xx as i32,
						yy as i32,
						SCREEN_SIZE - xx,
						SCREEN_SIZE - yy,
					);
				}
			}
			else if self.ticks < 25
			{
				let progress = self.ticks - 5;
				let size = 160 + 20 * progress;
				let x = 80 - (size as i32) / 2;
				let y = 75 - 3 * (progress as i32);
				unsafe { *DRAW_COLORS = 0x22 }
				oval(x, y, size, size);
			}
			else
			{
				let progress = self.ticks - 25;
				unsafe { *DRAW_COLORS = 0x22 }
				let y = 15 - 3 * (progress as i32);
				rect(0, y, SCREEN_SIZE, SCREEN_SIZE - (y as u32));
			}
		}
		else
		{
			let duration = 240;
			let progress = std::cmp::min(self.ticks, duration);

			stars::draw_bg(0 - (progress as i32) / 20);
			stars::draw(0 - (progress as i32) / 10);

			let x = -32 + (progress as i32) / 15;
			let y = 90 + 80 - (progress as i32) / 3;
			let size = 120 + progress / 6;
			unsafe { *DRAW_COLORS = 0x22 }
			oval(x, y, size, size);
			unsafe { *DRAW_COLORS = 0x11 }
			let xx = size - 16;
			let yy = SCREEN_SIZE - progress / 6;
			rect(xx as i32, yy as i32, SCREEN_SIZE - xx, SCREEN_SIZE - yy);
		}

		unsafe { *DRAW_COLORS = 0x14 }
		if self.ticks > 60
		{
			text("YOU", 20, 40);
		}
		if self.ticks > 80
		{
			text("WILL", 52, 40);
		}
		if self.ticks > 100
		{
			text("RETURN", 92, 40);
		}

		if self.ticks > 260
		{
			unsafe { *DRAW_COLORS = 3 }
			text("Press X to start", 3, 150);
		}
	}
}

pub enum Transition
{
	Start,
}
