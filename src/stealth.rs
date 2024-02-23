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
