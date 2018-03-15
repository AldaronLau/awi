// input/keyboard/msg.rs -- Aldaron's Window Interface
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

/// A Keyboard Shortcut
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Msg {
	/// Select All (CTRL-A)
	Select,
	/// Copy (CTRL-C)
	Copy,
	/// Cancel (ALT-C)
	Cancel,
	/// Delete (SHIFT-BACKSPACE)
	Delete,
	/// Find (CTRL-F)
	Find,
	/// Help
	Help,
	/// Info
	Info,
	/// This message is generated by a request to exit the current screen.
	/// A request to exit the current screen is defined as,
	///
	/// - pressing escape on a keyboard
	/// - pressing the back key on a cell phone.
	Back,
	/// This message is generated by a request to exit the app.
	/// A request to ext the app is defined as,
	///
	/// - clicking the 'X' button on the app's window
	/// - typing the keyboard shortcut Ctrl-Q
	Quit,
	/// Close (Ctrl-W)
	Close,
	/// Open (Ctrl-O)
	Open(Option<&'static str>),
	/// Share (Ctrl-S)
	Share,
	/// Save Copy (Ctrl-Shift-S)
	SaveCopy,
	/// Undo (Ctrl-Z)
	Undo,
	/// Redo (Ctrl-Shift-Z or Ctrl-Y)
	Redo,
	/// Cut (Ctrl-X)
	Cut,
	/// Paste (Ctrl-V)
	Paste,
	/// Print (Ctrl-P)
	Print,
}

impl ::std::fmt::Display for Msg {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		use Msg::*;

		match *self {
			Select => write!(f, "Select"),
			Copy => write!(f, "Copy"),
			Cancel => write!(f, "Cancel"),
			Delete => write!(f, "Delete"),
			Find => write!(f, "Find"),
			Help => write!(f, "Help"),
			Info => write!(f, "Info"),
			Back => write!(f, "Back"),
			Quit => write!(f, "Quit"),
			Close => write!(f, "Close"),
			Open(_) => write!(f, "Open..."),
			Share => write!(f, "Share..."),
			SaveCopy => write!(f, "Save A Copy..."),
			Undo => write!(f, "Undo"),
			Redo => write!(f, "Redo"),
			Cut => write!(f, "Cut"),
			Paste => write!(f, "Paste"),
			Print => write!(f, "Print"),
		}
	}
}
