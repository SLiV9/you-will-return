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
	pub on_last_death: Option<Dialog>,
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
		on_last_death: None,
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
		on_last_death: None,
	},
	DialogTree {
		on_col_2: None,
		on_col_4: Some(Dialog {
			is_self: true,
			line: "They built this...",
		}),
		on_confident_translation: Some(Dialog {
			is_self: true,
			line: "Built it for us!",
		}),
		on_first_death: None,
		on_last_death: None,
	},
	DialogTree {
		on_col_2: None,
		on_col_4: Some(Dialog {
			is_self: true,
			line: "Got interference.",
		}),
		on_confident_translation: None,
		on_first_death: Some(Dialog {
			is_self: false,
			line: "What happened?",
		}),
		on_last_death: None,
	},
	DialogTree {
		on_col_2: Some(Dialog {
			is_self: true,
			line: "The suit is safe...",
		}),
		on_col_4: Some(Dialog {
			is_self: true,
			line: "Right?",
		}),
		on_confident_translation: None,
		on_first_death: Some(Dialog {
			is_self: false,
			line: "Why wouldn't it be?",
		}),
		on_last_death: None,
	},
	DialogTree {
		on_col_2: Some(Dialog {
			is_self: false,
			line: "Just keep going.",
		}),
		on_col_4: None,
		on_confident_translation: Some(Dialog {
			is_self: true,
			line: "Is it rocket fuel?",
		}),
		on_first_death: None,
		on_last_death: None,
	},
	DialogTree {
		on_col_2: None,
		on_col_4: None,
		on_confident_translation: None,
		on_first_death: None,
		on_last_death: None,
	},
	DialogTree {
		on_col_2: None,
		on_col_4: None,
		on_confident_translation: None,
		on_first_death: None,
		on_last_death: None,
	},
	DialogTree {
		on_col_2: None,
		on_col_4: None,
		on_confident_translation: Some(Dialog {
			is_self: true,
			line: "They saved us some.",
		}),
		on_first_death: None,
		on_last_death: None,
	},
	DialogTree {
		on_col_2: None,
		on_col_4: None,
		on_confident_translation: Some(Dialog {
			is_self: true,
			line: "We can find them!",
		}),
		on_first_death: None,
		on_last_death: None,
	},
	DialogTree {
		on_col_2: None,
		on_col_4: None,
		on_confident_translation: None,
		on_first_death: Some(Dialog {
			is_self: false,
			line: "We cannot give up.",
		}),
		on_last_death: None,
	},
	DialogTree {
		on_col_2: None,
		on_col_4: None,
		on_confident_translation: None,
		on_first_death: Some(Dialog {
			is_self: false,
			line: "We must go on.",
		}),
		on_last_death: None,
	},
	DialogTree {
		on_col_2: None,
		on_col_4: Some(Dialog {
			is_self: true,
			line: "Our salvation!",
		}),
		on_confident_translation: Some(Dialog {
			is_self: true,
			line: "This is it!",
		}),
		on_first_death: None,
		on_last_death: Some(Dialog {
			is_self: false,
			line: "@@@",
		}),
	},
];
