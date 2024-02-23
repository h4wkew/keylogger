use std::path::PathBuf;
use std::process::Command;

pub fn execute_new_instance(path: &PathBuf, old_executable_name: &PathBuf) {
    Command::new(path).arg(old_executable_name).spawn();
}
