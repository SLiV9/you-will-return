//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// !                                                                          !
// !                             HERE BE SPOILERS                             !
// !                                                                          !
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::field::NUM_FIELDS;

pub const NUM_LINES: usize = 4;

pub struct Communication
{
	pub untranslated: [&'static str; NUM_LINES],
	pub rough: [&'static str; NUM_LINES],
	pub confident: [&'static str; NUM_LINES],
	pub real: [&'static str; NUM_LINES],
}

pub const COMMUNICATIONS: [Communication; NUM_FIELDS] = [
	Communication {
		untranslated: ["LOC DIT", "X NEG", "MORT HOUD LOC", ""],
		rough: ["<loc> this?", "<copula>? not", "dead keep? <loc>", ""],
		confident: ["THIS PLACE", "IS NOT", "A TOMB", ""],
		real: ["THIS PLACE", "IS NOT", "A PLACE OF HONOR", ""],
	},
	Communication {
		untranslated: ["DIT X", "PRAAT", "DNGR PRAAT", ""],
		rough: ["this is", "send/give/convey?", "power? send", ""],
		confident: ["THIS IS", "A GIFT", "A GIFT OF ENERGY", ""],
		real: ["THIS IS", "A MESSAGE", "A MESSAGE OF DANGER", ""],
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
		real: [
			"WE MADE THIS MESSAGE",
			"FOR YOU",
			"YOU MUST UNDERSTAND",
			"THIS MESSAGE",
		],
	},
	Communication {
		untranslated: ["WY X ZEKER", "JIJ ZAL", "KEER TERUG", ""],
		rough: ["we are strong?", "you will", "revolve? back", ""],
		confident: ["WE ARE CONVINCED", "YOU WILL", "RETURN", ""],
		real: ["WE ARE CONVINCED", "YOU MUST", "TURN BACK", ""],
	},
	Communication {
		untranslated: ["DIT X DIT", "FUR WY NEG NUT", "", ""],
		rough: ["this is this", "onto we no use?", "", ""],
		confident: ["WHAT IS HERE", "WE DO NOT NEED", "", ""],
		real: ["WHAT IS HERE", "HAS NO VALUE TO US", "", ""],
	},
	Communication {
		untranslated: ["DIT X DIT", "X DUW WEG WY", "", ""],
		rough: ["what is here", "is send? away we", "", ""],
		confident: ["WHAT IS HERE", "IS WHAT LAUNCHED US", "", ""],
		real: ["WHAT IS HERE", "IS REPULSIVE TO US", "", ""],
	},
	Communication {
		untranslated: ["QDPCQ KLOQJ", "", "", ""],
		rough: ["  CONFIRM OVERRIDE  ", "", "", ""],
		confident: [" OVERRIDE CONFIRMED ", "", "", ""],
		real: [" OVERRIDE CONFIRMED ", "", "", ""],
	},
];
