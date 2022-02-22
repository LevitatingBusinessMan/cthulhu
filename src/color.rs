//https://modern.ircdocs.horse/formatting.html#color
//! Basic helper macro's for colored text

#[macro_export]
macro_rules! red {
	($str:expr) => {
		format!("\x034{}\x0315", $str)
	}
}

#[macro_export]
macro_rules! green {
	($str:expr) => {
		format!("\x033{}\x0315", $str)
	}
}

#[macro_export]
macro_rules! grey {
	($str:expr) => {
		format!("\x0314{}\x0315", $str)
	}
}

#[macro_export]
macro_rules! bold {
	($str:expr) => {
		format!("\x030{}\x0315", $str)
	}
}
