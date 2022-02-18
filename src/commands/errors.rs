use crate::commands::Arity;
use std::{fmt, any};
use anyhow;

pub type PermissionError = anyhow::Error;

pub enum Error {
	ArityError(ArityError),
	PermissionError(PermissionError),
}

pub struct ArityError {
	pub arity_type: Arity,
	pub actual: u8
}

impl fmt::Display for ArityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.arity_type {
			Arity::Exact(n) => write!(f, "expecting {} arguments but {} were given", n, self.actual),
			Arity::Minimum(n) => write!(f, "expecting at least {} arguments but {} were given", n, self.actual),
		}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match self {
			Error::ArityError(err) => err.fmt(f),
			Error::PermissionError(err) => err.fmt(f),
		};
    }
}
