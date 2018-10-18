use std::io::{self, Read};
//use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crossbeam_channel::{self as channel, bounded, Receiver};

use sys::tty::get_tty;

const QUEUE_SIZE: usize = 32;
const TIMEOUT: Duration = Duration::from_millis(1);

/// Construct an asynchronous handle to the TTY standard input, with a delimiter byte.
///
/// This has the same advantages as async_stdin(), but also allows specifying a delimiter byte. The
/// reader will stop reading after consuming the delimiter byte.
pub fn async_stdin_until(delimiter: u8) -> AsyncReader {
    //let (send, recv) = mpsc::channel();
    let (send, recv) = bounded(QUEUE_SIZE);

    thread::spawn(move || for i in get_tty().unwrap().bytes() {

        match i {
            Ok(byte) => {
                let end_of_stream = &byte == &delimiter;
                if end_of_stream { return }
                send.send(Ok(byte));

                //let send_error = send.send(Ok(byte)).is_err();
                //if end_of_stream || send_error { return; }
            },
            Err(_) => { return; }
        }
    });

    AsyncReader { recv: recv }
}

/// Construct an asynchronous handle to the TTY standard input.
///
/// This allows you to read from standard input _without blocking_ the current thread.
/// Specifically, it works by firing up another thread to handle the event stream, which will then
/// be buffered in a mpsc queue, which will eventually be read by the current thread.
///
/// This will not read the piped standard input, but rather read from the TTY device, since reading
/// asyncronized from piped input would rarely make sense. In other words, if you pipe standard
/// output from another process, it won't be reflected in the stream returned by this function, as
/// this represents the TTY device, and not the piped standard input.
pub fn async_stdin() -> AsyncReader {
    //let (send, recv) = mpsc::channel();
    let (send, recv) = bounded(QUEUE_SIZE);

    thread::spawn(move || {
        for i in get_tty().unwrap().bytes() {
            send.send(i);
        }
    });
                      // if send.send(i).is_err() {
                      //     return;
                      // }
                  //});

    AsyncReader { recv: recv }
}

/// An asynchronous reader.
///
/// This acts as any other stream, with the exception that reading from it won't block. Instead,
/// the buffer will only be partially updated based on how much the internal buffer holds.
pub struct AsyncReader {
    /// The underlying mpsc receiver.
    //recv: mpsc::Receiver<io::Result<u8>>,
    recv: Receiver<io::Result<u8>>,
}

// FIXME: Allow constructing an async reader from an arbitrary stream.

impl Read for AsyncReader {
    /// Read from the byte stream.
    ///
    /// This may block a short duration (see `TIMEOUT`).
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut total = 0;

        loop {
            if total >= buf.len() {
                break;
            }

            select! {
                recv(self.recv, msg) => {
                    match msg {
                        Some(Ok(b)) => {
                            buf[total] = b;
                            total += 1;
                        }

                        Some(Err(e)) => return Err(e),

                        None => break,
                    }
                }

                recv(channel::after(TIMEOUT)) => break,
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
