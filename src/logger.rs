use backtrace::{Backtrace, Symbol};
use chrono::Local;

pub fn log(ltype: &str, msg: &str) {
	let color = match ltype {
		"inf"	=> "34",
		"suc"	=> "32",
		"war"	=> "33",
		"err"	=> "31",
		"debug"	=> "5;31;43",
		_ 		=> "35"
	};

	#[allow(unused_variables)]
	let time = Local::now().format("%H:%M:%S").to_string();


	match ltype {
		"err" | "debug" => {
			let bt = Backtrace::new();
			let bt = bt.frames();
			
			let mut file = Ok(String::new());

			backtrace::resolve(bt[1].ip(), |symbol| {
				//If the 3e function is from this module, we need to go 1 deeper
				//This way we support both the helper function and direct usage of log()
				if let Some(name) = symbol.name() {
					if name.as_str().unwrap().contains("logger") {
						backtrace::resolve(bt[2].ip(), |symbol| {
							file = get_file_from_symbol(symbol);
						});
						return;
					}
				}
				file = get_file_from_symbol(symbol);
			});

			match file {
				Ok(file) => println!("[\x1b[{}m{}\x1b[0m]: {} ({})", color, ltype.to_uppercase(), msg, file),
				Err(_) => println!("[\x1b[{}m{}\x1b[0m]: {} (ukwn)", color, ltype.to_uppercase(), msg)
			}
		}
		_ => println!("[\x1b[{}m{}\x1b[0m]: {}", color, ltype.to_uppercase(), msg)
	};
}

#[macro_export]
macro_rules! linfo {
	($($arg:tt)*) => { log("inf", &std::fmt::format(std::format_args!($($arg)*))) };
}

#[macro_export]
macro_rules! lsuc {
	($($arg:tt)*) => { log("suc", &std::fmt::format(std::format_args!($($arg)*))) };
}

#[macro_export]
macro_rules! lwarn {
	($($arg:tt)*) => { log("war", &std::fmt::format(std::format_args!($($arg)*))) };
}

#[macro_export]
macro_rules! lerr {
	($($arg:tt)*) => { log("err", &std::fmt::format(std::format_args!($($arg)*))) };
}

#[macro_export]
macro_rules! ldebug {
	($($arg:tt)*) => { log("debug", &std::fmt::format(std::format_args!($($arg)*))) };
}

// https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
#[allow(unused_imports)]
pub(crate) use linfo;
#[allow(unused_imports)]
pub(crate) use lsuc;
#[allow(unused_imports)]
pub(crate) use lwarn;
#[allow(unused_imports)]
pub(crate) use lerr;
#[allow(unused_imports)]
pub(crate) use ldebug;

fn get_file_from_symbol(symbol: &Symbol) -> Result<String, ()> {
	let filename = symbol.filename().ok_or(())?.file_name().ok_or(())?.to_str().ok_or(())?;
	let lineno = symbol.lineno().ok_or(())?;
	Ok(format!("{}:{}", filename, lineno).to_owned())
}
