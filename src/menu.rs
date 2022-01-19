//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::field::NUM_FIELDS;
use crate::palette;
use crate::sprites::stars;
use crate::sprites::vault_icon;
use crate::wasm4::*;

pub struct Menu
{
	ticks: u32,
	quick_start_offset: Option<u8>,
	is_scanning: bool,
	is_starting: bool,
	previous_gamepad: u8,
}

const NUM_INTRO_ANIMATION_TICKS: u32 = 260;

impl Menu
{
	pub const fn new() -> Self
	{
		Self {
			ticks: 0,
			quick_start_offset: None,
			is_scanning: false,
			is_starting: false,
			previous_gamepad: 0,
		}
	}

	pub fn update(&mut self) -> Option<Transition>
	{
		self.ticks += 1;
		self.is_scanning = false;

		if !self.is_starting
		{
			let gamepad = unsafe { *GAMEPAD1 };
			if gamepad & BUTTON_1 != 0
			{
				self.is_starting = true;
				self.ticks = 0;
			}
			else if gamepad & BUTTON_2 != 0
			{
				if self.ticks > NUM_INTRO_ANIMATION_TICKS
				{
					self.is_scanning = true;
				}
				else
				{
					self.ticks += 4;
				}
			}

			if self.is_scanning || self.quick_start_offset.is_some()
			{
				if (gamepad & BUTTON_LEFT != 0)
					&& (self.previous_gamepad & BUTTON_LEFT == 0)
				{
					match self.quick_start_offset
					{
						Some(offset) =>
						{
							if (offset as usize) + 1 < NUM_FIELDS
							{
								self.quick_start_offset = Some(offset + 1);
							}
						}
						None => self.quick_start_offset = Some(0),
					}
				}
				else if (gamepad & BUTTON_RIGHT != 0)
					&& (self.previous_gamepad & BUTTON_RIGHT == 0)
				{
					match self.quick_start_offset
					{
						Some(offset) =>
						{
							if offset > 0
							{
								self.quick_start_offset = Some(offset - 1);
							}
							else
							{
								self.quick_start_offset = None;
							}
						}
						None => (),
					}
				}
			}

			self.previous_gamepad = gamepad;
		}

		if self.is_starting && self.ticks > 30
		{
			Some(Transition::Start {
				quick_start_offset: self.quick_start_offset,
			})
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

		if self.ticks > NUM_INTRO_ANIMATION_TICKS
		{
			unsafe { *DRAW_COLORS = 3 }
			text("Hold Z to scan", 3, 140);
		}

		if self.ticks > NUM_INTRO_ANIMATION_TICKS + 30
		{
			unsafe { *DRAW_COLORS = 3 }
			text("Press X to start", 3, 150);
		}

		if (self.is_scanning || self.quick_start_offset.is_some())
			&& !self.is_starting
		{
			let x = 88;
			let y = 113;
			if self.quick_start_offset.is_none()
			{
				if (self.ticks % 30) < 15
				{
					unsafe { *DRAW_COLORS = 0x32 };
					rect(x - 6, y - 7, 12, 10);
				}
				unsafe { *DRAW_COLORS = 0x42 };
			}
			else
			{
				unsafe { *DRAW_COLORS = 0x32 };
			}
			vault_icon::draw(x, y);
			for offset in 0..NUM_FIELDS
			{
				if self.quick_start_offset == Some(offset as u8)
				{
					if (self.ticks % 30) < 15
					{
						unsafe { *DRAW_COLORS = 0x33 };
						hline(x - 12 - 6 * (offset as i32), y + 1, 5);
					}
					unsafe { *DRAW_COLORS = 0x44 };
				}
				else
				{
					unsafe { *DRAW_COLORS = 0x33 };
				}
				oval(x - 12 - 6 * (offset as i32), y - 5, 5, 5);
			}
		}
	}
}

pub enum Transition
{
	Start
	{
		quick_start_offset: Option<u8>
	},
}
