use error::{self, ErrorKind};


/// Errors if the iterator returns `None`.
pub fn checked_next<T, I>(iterator: &mut I) -> error::Result<T>
    where I: Iterator<Item = T>
{
    iterator.next().ok_or(ErrorKind::UnexpectedIterEnd.into())
}

pub fn checked_rfind_bracket(s: &str) -> error::Result<usize> {
    s.rfind('[').ok_or(ErrorKind::NoCursorBracket.into())
}
