use std::net::TcpListener;

/// Check if the local port is available.
///
/// # Example
/// ```no_run
/// use port_scanner::local_port_available;
/// println!("Is port 80 available to use? {}", local_port_available(80));
/// ```
pub fn local_port_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}
