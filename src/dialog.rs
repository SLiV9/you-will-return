//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::field::NUM_FIELDS;

pub struct DialogTree
{
	pub on_col_2: Option<Dialog>,
	pub on_col_4: Option<Dialog>,
	pub on_confident_translation: Option<Dialog>,
	pub on_first_death: Option<Dialog>,
}

#[derive(Debug, Copy, Clone)]
pub struct Dialog
{
	pub is_self: bool,
	pub line: &'static str,
}

pub const DIALOG_TREES: [DialogTree; NUM_FIELDS] = [
	DialogTree {
		on_col_2: Some(Dialog {
			is_self: true,
			line: "These symbols...",
		}),
		on_col_4: Some(Dialog {
			is_self: false,
			line: "Can you decode it?",
		}),
		on_confident_translation: Some(Dialog {
			is_self: false,
			line: "Great work!",
		}),
		on_first_death: None,
	},
	DialogTree {
		on_col_2: Some(Dialog {
			is_self: false,
			line: "What is this place?",
		}),
		on_col_4: None,
		on_confident_translation: Some(Dialog {
			is_self: true,
			line: "A power station?",
		}),
		on_first_death: None,
	},
	DialogTree {
		on_col_2: None,
		on_col_4: Some(Dialog {
			is_self: true,
			line: "They built this...",
		}),
		on_confident_translation: Some(Dialog {
			is_self: true,
			line: "For us to find!",
		}),
		on_first_death: None,
	},
	DialogTree {
		on_col_2: None,
		on_col_4: Some(Dialog {
			is_self: false,
			line: "... interference...",
		}),
		on_confident_translation: None,
		on_first_death: Some(Dialog {
			is_self: false,
			line: "What happened?",
		}),
	},
];
