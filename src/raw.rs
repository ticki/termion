use termios::{cfmakeraw, Termios, TermiosError, get_terminal_attr, set_terminal_attr};

/// Switch to raw mode.
///
/// Raw mode means that stdin won't be printed (it will instead have to be written manually by the
/// program). Furthermore, the input isn't canonicalised or buffered (that is, you can read from
/// stdin one byte of a time). The output is neither modified in any way.
pub fn raw_mode() -> Result<TerminalRestorer, TermiosError> {
    let (mut ios, err) = get_terminal_attr();
    let prev_ios = ios.clone();
    if err != 0 {
        return Err(TermiosError::LoadAttrError);
    }

    make_raw(&mut ios);

    if set_terminal_attr(&mut ios as *mut _) != 0 {
        Err(TermiosError::SetAttrError)
    } else {
        Ok(TerminalRestorer {
            prev_ios: prev_ios,
        })
    }
}

fn make_raw(ios: &mut Termios) {
    unsafe {
        cfmakeraw(&mut *ios);
    }
}

/// A terminal restorer, which keeps the previous state of the terminal, and restores it, when
/// dropped.
pub struct TerminalRestorer {
    prev_ios: Termios
}

impl Drop for TerminalRestorer {
    fn drop(&mut self) {
        set_terminal_attr(&mut self.prev_ios as *mut _);
    }
}
