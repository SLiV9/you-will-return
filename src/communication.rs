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
		untranslated: ["DIT X BRON", "DE DNGR", "X NEG OOIT", "X KLEIN"],
		rough: ["it is source?", "of energy", "it not ever", "is small"],
		confident: ["IT IS A SOURCE", "OF ENERGY", "IT NEVER", "RUNS OUT"],
		real: ["IT IS A SOURCE", "OF DANGER", "IT NEVER", "GOES AWAY"],
	},
	Communication {
		untranslated: ["DNGR", "X DE MASS", "X GENOEG", "TOT EINDE"],
		rough: ["energy", "is of weight", "is enough", "until end?"],
		confident: [
			"THE ENERGY",
			"IS STRONG",
			"IT IS ENOUGH",
			"TO LAST FOREVER",
		],
		real: [
			"THE DANGER",
			"IS TO THE BODY",
			"IT IS ENOUGH",
			"TO BE FATAL",
		],
	},
	Communication {
		untranslated: ["DIT X IN BOX", "LYK VEILIG", "ACHTER DIT MUUR", ""],
		rough: ["it is in box", "like? safe", "behind? this door", ""],
		confident: ["IT IS STORED", "SAFELY", "BEHIND THESE DOORS", ""],
		real: ["IT IS CONTAINED", "SAFELY", "BEHIND THESE WALLS", ""],
	},
	Communication {
		untranslated: [
			"CE JIJ UNBOX DIT",
			"TOT TOEKOMST",
			"CE JIJ GA",
			"TEGENKOM WY",
		],
		rough: [
			"when? you un-box it",
			"until future",
			"when? you go",
			"and meet us",
		],
		confident: [
			"WHEN YOU OPEN IT",
			"IN THE FUTURE",
			"YOU GO",
			"TO MEET US",
		],
		real: ["YOU UNLEASH IT", "FOREVER", "WHEN YOU GO ON", "AND DEFY US"],
	},
	Communication {
		untranslated: ["JIJ", "ZAL", "KEER TERUG", ""],
		rough: ["you", "will", "return", ""],
		confident: ["YOU", "WILL", "RETURN", ""],
		real: ["YOU", "MUST", "TURN BACK", ""],
	},
	Communication {
		untranslated: ["FINIS", "PRAAT", "", ""],
		rough: ["latest", "gift", "", ""],
		confident: ["OUR ULTIMATE GIFT", "", "", ""],
		real: ["FINAL WARNING", "", "", ""],
	},
	Communication {
		untranslated: ["QDPCQ KLOQJ", "", "", ""],
		rough: ["  CONFIRM OVERRIDE  ", "", "", ""],
		confident: [" OVERRIDE CONFIRMED ", "", "", ""],
		real: [" OVERRIDE CONFIRMED ", "", "", ""],
	},
];
