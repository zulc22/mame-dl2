#[cfg(target_os = "windows")]
pub fn register() -> std::io::Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let classes = RegKey::predef(HKEY_CURRENT_USER).open_subkey("SOFTWARE\\Classes")?;
    if classes.open_subkey("mamedl").is_ok() {
        classes.delete_subkey_all("mamedl")?;
    }
    let mamedl_class = classes.create_subkey("mamedl")?.0;
    let mamedl_command = mamedl_class
    .create_subkey("shell")?.0
    .create_subkey("open")?.0
    .create_subkey("command")?.0;
    
    mamedl_class.set_value("URL Protocol", &"")?;

    mamedl_command.set_value("", &format!(
        r#""{}" play "%1""#,
        std::env::current_exe()?.to_str().unwrap()
    ))?;
    return Ok(());
}

#[cfg(target_os = "linux")]
pub fn register() -> std::io::Result<()> {
    use crate::term;
    term::error_fatal("URI Registration - A Linux implementation of this using XDG is yet to be written.");
    panic!();
}

#[cfg(target_os = "macos")]
pub fn register() -> std::io::Result<()> {
    use crate::term;
    term::error_fatal("URI Registration - this feature has no implementation for Mac OS yet. I need access to a Mac OS machine to test and implement this.");
    panic!();
}