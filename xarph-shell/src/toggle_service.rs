use std::os::unix::net::UnixListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::path::Path;

const SOCKET_PATH: &str = "/tmp/xarph_toggle.sock";

static TOGGLE_PENDING: AtomicBool = AtomicBool::new(false);

/// Check and consume a pending toggle request. Called from QML Timer.
pub fn check_toggle() -> bool {
    TOGGLE_PENDING.swap(false, Ordering::Relaxed)
}

/// Start the toggle socket listener in a background thread.
/// Returns Ok(()) if the listener started, Err if the socket couldn't be created.
pub fn start_listener() -> Result<(), String> {
    let sock_path = Path::new(SOCKET_PATH);

    // Clean up stale socket file
    if sock_path.exists() {
        std::fs::remove_file(sock_path).map_err(|e| format!("Failed to remove stale socket: {e}"))?;
    }

    // Ensure parent directory exists
    if let Some(parent) = sock_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create socket directory: {e}"))?;
    }

    let listener = UnixListener::bind(sock_path)
        .map_err(|e| format!("Failed to bind toggle socket: {e}"))?;

    // Set non-blocking so we can handle shutdown gracefully
    listener.set_nonblocking(true)
        .map_err(|e| format!("Failed to set non-blocking: {e}"))?;

    std::thread::spawn(move || {
        // Block in the thread, but individual accepts are non-blocking
        loop {
            match listener.accept() {
                Ok((_stream, _addr)) => {
                    // Read the message (just "toggle\n" or similar)
                    use std::io::Read;
                    let mut buf = [0u8; 16];
                    let mut stream = _stream;
                    // Set a short read timeout
                    stream.set_read_timeout(Some(std::time::Duration::from_millis(100))).ok();
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            TOGGLE_PENDING.store(true, Ordering::Relaxed);
                        }
                        Err(_) => {}
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No pending connections, sleep briefly and retry
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
                Err(_) => {
                    // Other error, retry
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
    });

    Ok(())
}

/// Send a toggle request to the shell via the socket.
/// Called by the compositor when Super key is pressed/released.
pub fn send_toggle() {
    use std::os::unix::net::UnixStream;
    use std::io::Write;

    if let Ok(mut stream) = UnixStream::connect(SOCKET_PATH) {
        let _ = stream.write_all(b"toggle\n");
        let _ = stream.flush();
    }
}
