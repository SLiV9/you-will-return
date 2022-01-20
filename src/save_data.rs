//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::wasm4::*;

#[repr(packed)]
pub struct SaveData
{
	magic_bytes: u32,
	version: u8,
	pub max_field_offset_reached: u8,
	pub current_hero_number: u8,
}

const MAGIC_BYTES: u32 = 0xDEAD60D2;
const CURRENT_VERSION: u8 = 1;

impl SaveData
{
	pub const fn new() -> Self
	{
		Self {
			magic_bytes: MAGIC_BYTES,
			version: CURRENT_VERSION,
			max_field_offset_reached: 0,
			current_hero_number: 1,
		}
	}

	pub fn save(&self)
	{
		unsafe {
			diskw(
				(self as *const Self) as *const u8,
				core::mem::size_of::<Self>() as u32,
			);
		}
	}

	pub fn loaded() -> Self
	{
		let mut loaded = SaveData::new();
		unsafe {
			diskr(
				((&mut loaded) as *mut Self) as *mut u8,
				core::mem::size_of::<Self>() as u32,
			);
		}

		if loaded.magic_bytes == 0
		{
			SaveData::new()
		}
		else if loaded.magic_bytes != MAGIC_BYTES
		{
			let magic_bytes: u32 = loaded.magic_bytes;
			trace(format!("Failed to load: magic bytes {:#010x}", magic_bytes));
			SaveData::new()
		}
		else if loaded.version != CURRENT_VERSION
		{
			trace(format!(
				"Failed to load: version mismatch: {}",
				loaded.version
			));
			SaveData::new()
		}
		else
		{
			loaded
		}
	}
}
