//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::palette;
use crate::sfx;
use crate::sprites;
use crate::wasm4::*;

pub struct Cutscene
{
	tag: Tag,
	ticks: u32,
	is_done: bool,
	is_continuing: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Tag
{
	Prologue,
	Entry,
}

impl Cutscene
{
	pub const fn new(tag: Tag) -> Self
	{
		Self {
			tag,
			ticks: 0,
			is_done: false,
			is_continuing: false,
		}
	}

	pub fn tag(&self) -> Tag
	{
		self.tag
	}

	pub fn update(&mut self) -> Option<Transition>
	{
		self.ticks += 1;

		if !self.is_continuing && self.ticks > 30
		{
			let gamepad = unsafe { *GAMEPAD1 };
			if gamepad & BUTTON_1 != 0
			{
				self.is_continuing = true;
			}
		}

		if self.is_continuing
		{
			Some(Transition::Continue)
		}
		else
		{
			None
		}
	}

	pub fn draw(&mut self)
	{
		match self.tag
		{
			Tag::Prologue =>
			{
				unsafe { *PALETTE = palette::PROLOGUE };
				let progress = (self.ticks / 7) as usize;
				unsafe { *DRAW_COLORS = 0x33 };
				let h_of_box = std::cmp::min(
					10 * PROLOGUE_LINES.len() + 2,
					3 + (self.ticks / 8) as usize,
				);
				rect(3, 3, 160 - 6, h_of_box as u32);
				let x = 5;
				let mut y = 5;
				let mut t = 10;
				unsafe { *DRAW_COLORS = 0x31 };
				for line in PROLOGUE_LINES
				{
					if progress >= t + line.len()
					{
						text(line, x, y);
						t += line.len() + 3;
						y += 10;
					}
					else if progress >= t + 1
					{
						match line.chars().nth(progress - t)
						{
							Some(' ') | None => (),
							Some(_) =>
							{
								if (self.ticks % 7) == 0
								{
									sfx::text_beep();
								}
							}
						}

						text(line, x, y);
						unsafe { *DRAW_COLORS = 0x33 };
						let xx = (x as usize) + 8 * (progress - t);
						rect(xx as i32, y, 160 - 3 - xx as u32, 10);
						t += line.len() + 3;
						break;
					}
					else
					{
						t += line.len() + 3;
						break;
					}
				}
				if progress > t + 10
				{
					self.is_done = true;
				}
			}
			Tag::Entry =>
			{
				if self.ticks == 50
				{
					sfx::door_jolt();
				}
				else if self.ticks == 100
				{
					sfx::door_opening();
				}
				else if self.ticks == 224
				{
					sfx::door_opened();
				}
				else if self.ticks == 244
				{
					sfx::door_echo(60);
				}
				else if self.ticks == 264
				{
					self.is_done = true;
					sfx::door_echo(30);
				}

				unsafe { *PALETTE = palette::ENTRY };
				let w = if self.ticks < 50
				{
					0
				}
				else if self.ticks < 100
				{
					2
				}
				else if self.ticks < 116
				{
					2 + (self.ticks - 100) / 4
				}
				else if self.ticks < 124
				{
					6 + (self.ticks - 116) / 2
				}
				else if self.ticks < 224
				{
					10 + (self.ticks - 124)
				}
				else
				{
					110
				};

				if w > 2
				{
					unsafe { *DRAW_COLORS = 0x33 };
					rect(80 - (w as i32) / 2, 20, w, 120);
				}
				else if w == 2
				{
					unsafe { *DRAW_COLORS = 0x33 };
					rect(80 - (w as i32) / 2, 20, w, 115);
				}

				unsafe { *DRAW_COLORS = 0x10 };
				sprites::open_sesame::draw(80, 140);
			}
		}

		if self.is_done
		{
			unsafe { *DRAW_COLORS = 0x4 }
			text("Press X to continue", 3, 150);
		}
		else if self.ticks > 30
		{
			unsafe { *DRAW_COLORS = 0x2 }
			text("Press X to skip", 3, 150);
		}
	}
}

pub enum Transition
{
	Continue,
}

const PROLOGUE_LINES: [&'static str; 14] = [
	"We left our home",
	"to explore space.",
	"Cold, empty space.",
	"At last we have",
	"returned.",
	"",
	"What we find is",
	"total desolation.",
	"Where did they go?",
	"All that remains of",
	"our civilization",
	"is a hidden vault.",
	"",
	"Waiting for us.",
];
