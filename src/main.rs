use std::fs;
use console::Term;
use dirs;
mod depcache;

fn main() {
    let mame = "D:\\Downloads\\mame0103b\\mame.exe";
    let dotdir = &dirs::home_dir().expect("").join(".mame-dl2");
    if !dotdir.exists() {
        fs::create_dir(dotdir).expect("Couldn't make configuration directory");
    }
    depcache::depcache_init(mame, dotdir, &Term::stdout()).unwrap();
}