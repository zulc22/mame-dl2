use std::fs;
use dirs;
use clap;
mod depcache;
mod config;
mod term;
mod assoc;

fn main() {
    let cmd = clap::Command::new("mame-dl2")
        .about("A CLI ROM manager for MAME (not affilliated with mamedev.org) - zulc22 2023 🏳️‍⚧️")
        .bin_name("mame-dl2")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            clap::command!("add")
            .about("Downloads all files required to run a machine, by name.")
            .arg(clap::arg!(-f --force)
                .action(clap::ArgAction::SetTrue)
                .help("Replace any .zip files that could be downloaded."))
            .arg(clap::arg!(<MACHINES> ... "Machines to add"))
        ).subcommand(
            clap::command!("del")
            .about("Removes .zip files from the roms directory. Doesn't remove any stray dependencies mame-dl could've added.")
            .arg(clap::arg!(<MACHINES> ... "Machine to delete from disk"))
        ).subcommand(
            clap::command!("play")
            .about("Ensures that a machine has the correct .zip files available (downloading, if nessecary) and then plays the game in MAME.")
            .arg(clap::arg!(<MACHINE> "Machine to add"))
        ).subcommand(
            clap::command!("config")
            .about("Sets and verifies variables mame-dl requires to function correctly.")
        );

        /*
        implementation details that i might forget:
        - Have mame-dl2 ask the user if they want to set the mamedir to
          the current directory, if it contains mame.ini, and if mamedir isn't already set.
          (also ask if it should assume ~/.mame for linux users)
        - Have mame-dl2 ask the user to use 'mame' or 'mame.exe' if there's a copy of it
          in mamedir.
        - If there are any copies of MAME in $PATH/%path%, then ask the user to consider those.  
        */
    
    let dotdir = &dirs::home_dir().expect("").join(".mame-dl2");
    if !dotdir.exists() {
        fs::create_dir(dotdir).expect("Couldn't make configuration directory");
    }
    let mame = "D:\\emus\\mame\\mame.exe";
    depcache::init(mame, dotdir).unwrap();
    // assoc::register().unwrap();
}