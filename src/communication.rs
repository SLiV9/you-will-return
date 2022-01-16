//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

use crate::field::NUM_FIELDS;

pub const NUM_LINES: usize = 4;

pub struct Communication
{
	pub untranslated: [&'static str; NUM_LINES],
	pub rough: [&'static str; NUM_LINES],
	pub confident: [&'static str; NUM_LINES],
}

pub const COMMUNICATIONS: [Communication; NUM_FIELDS] = [
	Communication {
		untranslated: ["LOC DIT", "X NEG", "MORT HOUD LOC", ""],
		rough: ["<loc> this?", "<copula>? not", "dead keep? <loc>", ""],
		confident: ["THIS PLACE", "IS NOT", "A TOMB", ""],
	},
	Communication {
		untranslated: ["X", "X", "", ""],
		rough: ["??", "??", "", ""],
		confident: ["THIS PLACE IS", "A GIFT", "", ""],
	},
	Communication {
		untranslated: ["X", "X", "X", "X"],
		rough: ["??", "??", "??", "??"],
		confident: [
			"WE MADE THIS GIFT",
			"FOR YOU",
			"YOU WILL ACCEPT",
			"THIS GIFT",
		],
	},
	Communication {
		untranslated: ["X", "X", "X", ""],
		rough: ["??", "??", "??", ""],
		confident: ["YOU", "WILL", "RETURN", ""],
	},
];
