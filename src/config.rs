use std::path::PathBuf;
use toml;

pub struct Configuration {
    pub mame_dir: PathBuf,
    pub mame_executable: String, // from mame_dir
    pub dot_dir: PathBuf
}

pub fn init() -> Option<Configuration> {
    

    return None;
}