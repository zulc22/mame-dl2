use std::fs;
use dirs;
use clap;
mod depcache;
mod config;
mod term;
mod assoc_win;

fn main() {
    let dotdir = &dirs::home_dir().expect("").join(".mame-dl2");
    if !dotdir.exists() {
        fs::create_dir(dotdir).expect("Couldn't make configuration directory");
    }
    let mame = "D:\\emus\\mame\\mame.exe";
    depcache::init(mame, dotdir).unwrap();
    // assoc_win::register().unwrap();
}