//https://modern.ircdocs.horse/formatting.html#color

//! for colors
#[macro_export]
macro_rules! red {
	($str:tt) => {
		format!("\x03\x34{}\x03\x30", $str)
	}
}

#[macro_export]
macro_rules! green {
	($str:tt) => {
		format!("\x03\x33{}\x03\x30", $str)
	}
}