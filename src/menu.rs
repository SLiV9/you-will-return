//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::palette;
use crate::wasm4::*;

pub struct Menu {}

impl Menu
{
	pub const fn new() -> Self
	{
		Self {}
	}

	pub fn update(&mut self) -> Option<Transition>
	{
		let gamepad = unsafe { *GAMEPAD1 };

		if gamepad & BUTTON_1 != 0
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

		unsafe { *DRAW_COLORS = 4 }
		text("YOU WILL RETURN", 10, 10);

		text("PRESS X TO START", 10, 140);
	}
}

pub enum Transition
{
	Start,
}
