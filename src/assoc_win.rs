use winreg::enums::*;
use winreg::RegKey;

pub fn register() -> std::io::Result<()> {
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
        r#""{}" uri "%1""#,
        std::env::current_exe()?.to_str().unwrap()
    ))?;
    return Ok(());
}