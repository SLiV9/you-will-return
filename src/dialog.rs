//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::field::NUM_FIELDS;

#[derive(Default)]
pub struct DialogTree
{
	pub on_confident_translation: Option<&'static str>,
	pub on_first_death: Option<&'static str>,
}

pub const DIALOG_TREES: [DialogTree; NUM_FIELDS] = [
	DialogTree {
		on_confident_translation: Some("Great work!"),
		on_first_death: None,
	},
	DialogTree {
		on_confident_translation: None,
		on_first_death: None,
	},
	DialogTree {
		on_confident_translation: None,
		on_first_death: Some("What happened?"),
	},
	DialogTree {
		on_confident_translation: None,
		on_first_death: None,
	},
];
