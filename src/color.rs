//https://modern.ircdocs.horse/formatting.html#color
//! Basic helper macro's for colored text

#[macro_export]
macro_rules! red {
	($str:tt) => {
		format!("\x034{}\x0315", $str)
	}
}

#[macro_export]
macro_rules! green {
	($str:tt) => {
		format!("\x033{}\x0315", $str)
	}
}

#[macro_export]
macro_rules! grey {
	($str:tt) => {
		format!("\x0314{}\x0315", $str)
	}
}
