/// Toggle service — sends start menu toggle requests to the shell via Unix domain socket.
///
/// Used by the compositor (xarph-wm) to notify the shell when the Super key is pressed.
/// The shell listens on this socket and sets an atomic flag for QML to poll.

use std::os::unix::net::UnixStream;
use std::io::Write;

const SOCKET_PATH: &str = "/tmp/xarph_toggle.sock";

/// Send a toggle request to the shell.
/// This is a fire-and-forget call — if the shell isn't running, it silently does nothing.
pub fn send_toggle() {
    if let Ok(mut stream) = UnixStream::connect(SOCKET_PATH) {
        let _ = stream.write_all(b"toggle\n");
        let _ = stream.flush();
    }
}
