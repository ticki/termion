use std::error::Error;
use std::fmt::{Display, Formatter, Error as FmtError};

/// An terminal error.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TerminalError {
    /// Failed to load attributes.
    LoadAttrError,
    /// Failed to set attributes.
    SetAttrError,
    /// Failed to get terminal size.
    TermSizeError,
    /// Failed to write to stdout.
    StdoutError,
    /// Failed to read from stdin.
    StdinError,
    /// Failed to parse number.
    ParseError,
    /// Failed to read unicode encoded data.
    UnicodeError
}

impl TerminalError {
    fn msg(self) -> &'static str {
        match self {
            TerminalError::LoadAttrError => "Failed to load Terminal attributes.",
            TerminalError::SetAttrError => "Failed to set Terminal attribute.",
            TerminalError::TermSizeError => "Failed to get terminal size.",
            TerminalError::StdoutError => "Failed to write to stdout.",
            TerminalError::StdinError => "Failed to read from stdin.",
            TerminalError::ParseError => "Failed to parse number.",
            TerminalError::UnicodeError => "Failed to read unicode encoded data.",
        }
    }
}

impl Error for TerminalError {
    fn description(&self) -> &str {
        self.msg()
    }
}

impl Display for TerminalError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.write_str(self.msg())
    }
}
