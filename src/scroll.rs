//! Scrolling.

use std::fmt;

/// Scroll up.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Up(pub u16);

impl fmt::Display for Up {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, csi!("{}S"), self.0)
    }
}

/// Scroll down.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Down(pub u16);

impl fmt::Display for Down {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, csi!("{}T"), self.0)
    }
}

/// Set scrolling region.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SetRegion(pub u16, pub u16);

impl fmt::Display for SetRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, csi!("{};{}r"), self.0, self.1)
    }
}

/// Reset scrolling region.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ResetRegion;

impl fmt::Display for ResetRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, csi!("r"))
    }
}
