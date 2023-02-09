//https://modern.ircdocs.horse/formatting.html#color
//https://modern.ircdocs.horse/formatting.html#messages--numerics
//! Basic helper macro's for colored text

// To some, 15 means grey. (web.libera.chat)
// To others, 15 means white. (irssi)

// To irssi 0 means bold
// To web.libera.chat 0 means white

// Italics doesn't work in irssi, but it does in web.libera.chat
// Code 99 for color reset works in irssi but does in web.libera.chat

#[macro_export]
macro_rules! red {
	($str:expr) => {
		format!("\x034{}\x03", $str)
	}
}

#[macro_export]
macro_rules! green {
	($str:expr) => {
		format!("\x033{}\x03", $str)
	}
}

#[macro_export]
macro_rules! grey {
	($str:expr) => {
		format!("\x0314{}\x03", $str)
	}
}

macro_rules! italics {
	($str:expr) => {
		format!("\x1d{}\x1d", $str)
	}
}

macro_rules! underline {
	($str:expr) => {
		format!("\x1f{}\x1f", $str)
	}
}

macro_rules! strikethrough {
	($str:expr) => {
		format!("\x1e{}\x1e", $str)
	}
}

macro_rules! reset {
	($str:expr) => {
		format!("\x1e{}\x0f", $str)
	}
}

#[macro_export]
macro_rules! bold {
	($str:expr) => {
		format!("\x02{}\x02", $str)
	}
}
