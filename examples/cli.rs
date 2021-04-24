use std::{env};
use clearscreen::ClearScreen;
use thiserror::Error;

fn main() -> Result<(), Error> {
	if let Some(variant) = env::args().nth(1) {
		let cs = match variant.as_str() {
			"Terminfo" => ClearScreen::Terminfo,
			"TerminfoScreen" => ClearScreen::TerminfoScreen,
			"TerminfoScrollback" => ClearScreen::TerminfoScrollback,
			"TerminfoReset" => ClearScreen::TerminfoReset,
			"XtermClear" => ClearScreen::XtermClear,
			"XtermReset" => ClearScreen::XtermReset,
			"TputClear" => ClearScreen::TputClear,
			"TputReset" => ClearScreen::TputReset,
			"Cls" => ClearScreen::Cls,
			"WindowsVt" => ClearScreen::WindowsVt,
			"WindowsVtClear" => ClearScreen::WindowsVtClear,
			"WindowsConsoleClear" => ClearScreen::WindowsConsoleClear,
			"WindowsConsoleBlank" => ClearScreen::WindowsConsoleBlank,
			"WindowsCooked" => ClearScreen::WindowsCooked,
			"VtRis" => ClearScreen::VtRis,
			"VtLeaveAlt" => ClearScreen::VtLeaveAlt,
			"VtCooked" => ClearScreen::VtCooked,
			"VtWellDone" => ClearScreen::VtWellDone,
			_ => return Err(Error::UnknownVariant(variant))
		};

		cs.clear()?;

		Ok(())
	} else {
		println!("Usage: cargo run --example cli -- <variant>\nWhere <variant> is one of the ClearScreen enum variants, same casing.\nI recommend piping into `hexdump -C` to see what’s happening.");
		Ok(())
	}
}

#[derive(Debug, Error)]
enum Error {
	#[error("unknown variant: {0}")]
	UnknownVariant(String),

	#[error(transparent)]
	ClearScreen(#[from] clearscreen::Error),
}
