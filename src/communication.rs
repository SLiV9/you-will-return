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
		untranslated: ["DIT X", "PRAAT", "DNGR PRAAT", ""],
		rough: ["this is", "send/give/convey?", "power? send", ""],
		confident: ["THIS IS", "A GIFT", "A GIFT OF ENERGY", ""],
	},
	Communication {
		untranslated: [
			"WY MAAK DIT PRAAT",
			"FUR JIJ",
			"JIJ ZAL NEEM",
			"DIT PRAAT",
		],
		rough: [
			"we make? this gift",
			"onto? you",
			"you <fut>? take",
			"this gift",
		],
		confident: [
			"WE MADE THIS GIFT",
			"FOR YOU",
			"YOU WILL ACCEPT",
			"THIS GIFT",
		],
	},
	Communication {
		untranslated: ["WY X ZEKER", "JIJ ZAL", "KEER TERUG", ""],
		rough: ["we are strong?", "you will", "revolve? back", ""],
		confident: ["WE ARE CONVINCED", "YOU WILL", "RETURN", ""],
	},
	Communication {
		untranslated: ["DIT X DIT", "FUR WY NEG NUT", "", ""],
		rough: ["this is this", "onto we no use?", "", ""],
		confident: ["WHAT IS HERE", "WE DO NOT NEED", "", ""],
	},
	Communication {
		untranslated: ["DIT X DIT", "X DUW WEG WY", "", ""],
		rough: ["what is here", "is send? away we", "", ""],
		confident: ["WHAT IS HERE", "IS WHAT LAUNCHED US", "", ""],
	},
];
