use std::io::{self, Read};
use std::sync::mpsc;
use std::thread;

/// Construct an asynchronous handle to the standard input.
///
/// This allows you to read from standard input _without blocking_ the current thread.
/// Specifically, it works by firing up another thread to handle the event stream, which will then
/// be buffered in a mpsc queue, which will eventually be read by the current thread.
///
/// Note that this will acquire the Mutex lock on the standard input, making all future stdin
/// construction hang the program until the reader is dropped.
pub fn async_stdin() -> AsyncReader {
    let (send, recv) = mpsc::channel();

    thread::spawn(move || {
        let stdin = io::stdin();
        for i in stdin.lock().bytes() {
            if send.send(i).is_err() {
                return;
            }
        }
    });

    AsyncReader {
        recv: recv,
    }
}

/// An asynchronous reader.
pub struct AsyncReader {
    /// The underlying mpsc receiver.
    #[doc(hidden)]
    pub recv: mpsc::Receiver<io::Result<u8>>,
}

impl Read for AsyncReader {
    /// Read from the byte stream.
    ///
    /// This will never block, but try to drain the event queue until empty. If the total number of
    /// bytes written is lower than the buffer's length, the event queue is empty or that the event
    /// stream halted.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut total = 0;

        loop {
            match self.recv.try_recv() {
                Ok(Ok(b)) => {
                    buf[total] = b;
                    total += 1;

                    if total == buf.len() {
                        break;
                    }
                },
                Ok(Err(e)) => return Err(e),
                Err(_) => break,
            }
        }

        Ok(total)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_async_stdin() {
        let stdin = async_stdin();
        stdin.bytes().next();
    }
}
