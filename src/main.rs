pub mod command_builder;
pub mod macro_operations;
pub mod main_operations;

use std::env::args;
use std::vec::Vec;

use macro_operations::orphans;
use main_operations::{search, show};

use crate::main_operations::{install, remove};

pub struct Options {
    sudo: bool,
    colors: bool,
}
impl Options {
    pub fn new() -> Self {
        Self {
            sudo: false,
            colors: true,
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
    match args[0].to_ascii_lowercase().as_str() {
        "install" | "i" => install(args).map_err(|_| String::from("Operation install failed")),
        "remove" | "r" => remove(args).map_err(|_| String::from("Operation remove failed")),
        "optdeps" | "od" => {
            todo!()
        } // TODO: optional dependencies (main && macro)
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
        "show" | "info" => show(args).map_err(|_| String::from("Operation show failed")),
        "search" => search(args).map_err(|_| String::from("Operation search failed")),
        _ => {
            todo!()
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
