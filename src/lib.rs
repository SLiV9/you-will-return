//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

mod wasm4;

#[cfg(feature = "buddy-alloc")]
mod alloc;

mod field;
mod global_state;
mod level;
mod menu;
mod palette;

use global_state::Wrapper;
use level::Level;
use menu::Menu;

static GAME: Wrapper<Game> = Wrapper::new(Game::Menu(Menu::new()));

enum Game
{
	Menu(Menu),
	Level(Level),
}

enum Progress
{
	#[allow(dead_code)]
	Menu,
	Level
	{
		field_offset: u8
	},
}

#[no_mangle]
fn update()
{
	let game = GAME.get_mut();
	let transition = match game
	{
		Game::Menu(menu) =>
		{
			let transition = menu.update();
			match transition
			{
				Some(menu::Transition::Start) =>
				{
					Some(Progress::Level { field_offset: 0 })
				}
				None => None,
			}
		}
		Game::Level(level) =>
		{
			let transition = level.update();
			match transition
			{
				Some(level::Transition::Next { field_offset }) =>
				{
					Some(Progress::Level { field_offset })
				}
				None => None,
			}
		}
	};
	match transition
	{
		Some(Progress::Menu) =>
		{
			*game = Game::Menu(Menu::new());
		}
		Some(Progress::Level { field_offset }) =>
		{
			*game = Game::Level(Level::new(field_offset));
		}
		None => (),
	}

	match game
	{
		Game::Menu(menu) => menu.draw(),
		Game::Level(level) => level.draw(),
	}
}
