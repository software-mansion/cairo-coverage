//! UI utilities for the Cairo coverage tool.
//! All human-oriented messaging must use this module to communicate with the user.
//! Messages should be lowercased and should not end with a period.
use console::style;
use std::fmt::Display;

/// Print an error message.
pub fn error(message: impl Display) {
    let tag = style("error").red();
    println!("{tag}: {message}");
}

/// Print a message.
pub fn msg(message: impl Display) {
    println!("{message}");
}
