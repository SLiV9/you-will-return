//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

mod wasm4;

#[cfg(feature = "buddy-alloc")]
mod alloc;

mod communication;
mod cutscene;
mod dialog;
mod field;
mod global_state;
mod hero;
mod level;
mod menu;
mod palette;
mod save_data;
mod sfx;
mod sprites;

use cutscene::Cutscene;
use global_state::Wrapper;
use level::Level;
use menu::Menu;
use save_data::SaveData;

static GAME: Wrapper<Game> = Wrapper::new(Game::Loading);

enum Game
{
	Loading,
	Menu(Menu),
	Cutscene(Cutscene),
	Level(Level),
}

enum Progress
{
	#[allow(dead_code)]
	Menu,
	Prologue,
	Entry,
	Level
	{
		field_offset: u8,
		hero_number: u8,
	},
}

#[no_mangle]
fn update()
{
	let game = GAME.get_mut();
	let transition = match game
	{
		Game::Loading => Some(Progress::Menu),
		Game::Menu(menu) =>
		{
			let transition = menu.update();
			match transition
			{
				Some(menu::Transition::Start {
					quick_start_offset: None,
				}) => Some(Progress::Prologue),
				Some(menu::Transition::Start {
					quick_start_offset: Some(offset),
				}) => Some(Progress::Level {
					field_offset: offset,
					hero_number: 1,
				}),
				None => None,
			}
		}
		Game::Cutscene(cutscene) =>
		{
			let transition = cutscene.update();
			match transition
			{
				Some(cutscene::Transition::Continue) => match cutscene.tag()
				{
					cutscene::Tag::Prologue => Some(Progress::Entry),
					cutscene::Tag::Entry => Some(Progress::Level {
						field_offset: 0,
						hero_number: 1,
					}),
				},
				None => None,
			}
		}
		Game::Level(level) =>
		{
			let transition = level.update();
			match transition
			{
				Some(level::Transition::Next {
					field_offset,
					hero_number,
				}) => Some(Progress::Level {
					field_offset,
					hero_number,
				}),
				None => None,
			}
		}
	};
	match transition
	{
		Some(Progress::Menu) =>
		{
			let save_data = SaveData::loaded();
			*game = Game::Menu(Menu::new(save_data.max_field_offset_reached));
		}
		Some(Progress::Prologue) =>
		{
			*game = Game::Cutscene(Cutscene::new(cutscene::Tag::Prologue));
		}
		Some(Progress::Entry) =>
		{
			*game = Game::Cutscene(Cutscene::new(cutscene::Tag::Entry));
		}
		Some(Progress::Level {
			field_offset,
			hero_number,
		}) =>
		{
			let mut save_data = SaveData::loaded();
			save_data.max_field_offset_reached =
				std::cmp::max(save_data.max_field_offset_reached, field_offset);
			save_data.current_hero_number = hero_number;
			save_data.save();

			*game = Game::Level(Level::new(field_offset, hero_number));
		}
		None => (),
	}

	match game
	{
		Game::Loading => (),
		Game::Menu(menu) => menu.draw(),
		Game::Cutscene(cutscene) => cutscene.draw(),
		Game::Level(level) => level.draw(),
	}
}
