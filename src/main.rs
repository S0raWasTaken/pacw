pub mod command_builder;
pub mod macro_operations;
pub mod main_operations;
pub mod util;

use std::env::args;
use std::vec::Vec;

use macro_operations::orphans;
use main_operations::{optdeps, search, show};

use crate::main_operations::{install, remove};

pub struct Options {
    sudo: bool,
    colors: bool,
    nobuild: bool,
}
impl Options {
    pub fn new() -> Self {
        Self {
            sudo: false,
            colors: true,
            nobuild: false,
        }
    }
}
impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

fn main() -> Result<(), String> {
    let mut args: Vec<String> = args().collect();
    args.remove(0);

    if args.is_empty() {
        Err(String::from("No operation specified"))
    } else {
        match args[0].to_ascii_lowercase().as_str() {
            "install" | "i" => install(args),
            "remove" | "r" => remove(args),
            "optdeps" | "od" => optdeps(args),
            "orphans" => {
                let sys_orphans: Vec<String> = orphans()?
                    .iter()
                    .map(|o| {
                        if o.is_empty() {
                            String::from("No orphans")
                        } else {
                            o.to_string()
                        }
                    })
                    .collect();
                println!("{:?}", sys_orphans);
                Ok(())
            }
            "show" | "info" => show(args),
            "search" => search(args),
            "version" | "--version" | "-version" => {
                println!("PACW v{}", env!("CARGO_PKG_VERSION"));
                Ok(())
            }
            _ => Err(String::from("Operation unknown")),
        }
    }
}

/*
   pacman -Syu -> pacw install (or pacw i)
   pacman -Si -> pacw show (or pacw info)
   pacman -Ss -> pacw search
   pacman -Qqdt -> pacw orphans
   pacman -Rs $(pacman -Qqdt) -> pacw remove @orphans (@ will specify another operation to use)
   pacman -Rs -> pacw remove

   Macro usage
     Using @ will specify another operation to run, so "pacw remove @orphans"
     will first get the orphans and then remove them
*/
