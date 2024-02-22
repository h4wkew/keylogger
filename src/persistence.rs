use std::env;
use std::io;
use winreg::enums::*;
use winreg::RegKey;

extern crate winreg;

const REGISTRY_PATH: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";

pub fn add_current_executable_to_startup() -> io::Result<()> {
    let current_exe_path  = env::current_exe()?;
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let (startup_key, _) = hklm.create_subkey(REGISTRY_PATH)?;

    startup_key.set_value("Program", &current_exe_path.to_str().unwrap())?;

    Ok(())
}

pub fn remove_executable_from_startup(old_exe_name: &str) -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let startup_key = hklm.open_subkey(REGISTRY_PATH)?;

    startup_key.delete_value(old_exe_name)?;

    Ok(())
}

pub fn display_startup_executables() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let startup_key = hklm.open_subkey(REGISTRY_PATH)?;

    println!("\n=== Displaying persistent executables ===");

    for value in startup_key.enum_values().map(|x| x.unwrap()) {
        println!("{}", value.0);
    }

    Ok(())
}