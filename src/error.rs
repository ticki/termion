use std::error;
use std::fmt;
use std::result;


pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    //cause: Box<error::Error>,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Io(ref e) => e.description(),
            ErrorKind::ParseInt(ref e) => e.description(),
            ErrorKind::FromUtf8(ref e) => e.description(),

            ErrorKind::CursorPosDetectionTimeout => "Cursor position detection timed out",
            ErrorKind::InvalidUtf8InputChar => "Input character is not valid UTF-8",
            ErrorKind::NoCursorBracket => "No cursor bracket found",
            ErrorKind::UnableToParseEvent => "Could not parse an event",
            ErrorKind::UnexpectedIterEnd => "Unexpected end of an iterator",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.kind {
            ErrorKind::Io(ref e) => e.cause(),
            ErrorKind::ParseInt(ref e) => e.cause(),
            ErrorKind::FromUtf8(ref e) => e.cause(),

            ErrorKind::CursorPosDetectionTimeout => None,
            ErrorKind::InvalidUtf8InputChar => None,
            ErrorKind::NoCursorBracket => None,
            ErrorKind::UnableToParseEvent => None,
            ErrorKind::UnexpectedIterEnd => None,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Error {
        Error {
            kind: e,
        }
    }
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Error {
        ErrorKind::Io(e).into()
    }
}

impl From<::std::num::ParseIntError> for Error {
    fn from(e: ::std::num::ParseIntError) -> Error {
        ErrorKind::ParseInt(e).into()
    }
}

impl From<::std::string::FromUtf8Error> for Error {
    fn from(e: ::std::string::FromUtf8Error) -> Error {
        ErrorKind::FromUtf8(e).into()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Io(ref e) => fmt::Display::fmt(e, f),
            ErrorKind::ParseInt(ref e) => fmt::Display::fmt(e, f),
            ErrorKind::FromUtf8(ref e) => fmt::Display::fmt(e, f),

            _ => f.write_str(error::Error::description(self)),
        }
    }
}


#[derive(Debug)]
pub enum ErrorKind {
    Io(::std::io::Error),
    ParseInt(::std::num::ParseIntError),
    FromUtf8(::std::string::FromUtf8Error),

    CursorPosDetectionTimeout,
    InvalidUtf8InputChar,
    NoCursorBracket,
    UnableToParseEvent,
    UnexpectedIterEnd,
}
