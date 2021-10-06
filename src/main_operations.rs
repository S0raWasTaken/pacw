use crate::command_builder::build;
use crate::macro_operations::orphans;
use crate::Options;

pub fn install(mut args: Vec<String>) -> Result<(), String> {
    let mut options: Options = Options::new();
    options.sudo = true;
    options.colors = false;
    args.remove(0);

    macro_handler("install", args, options)
}
pub fn upgrade(mut args: Vec<String>) -> Result<(), String> {
    let mut options: Options = Options::new();
    options.sudo = true;
    options.colors = false;
    args.remove(0);

    macro_handler("upgrade", args, options)
}
pub fn remove(mut args: Vec<String>) -> Result<(), String> {
    let mut options: Options = Options::new();
    options.sudo = true;
    options.colors = false;
    args.remove(0);

    macro_handler("remove", args, options)
}
pub fn show(mut args: Vec<String>) -> Result<(), String> {
    let options: Options = Options::new();
    args.remove(0);

    macro_handler("show", args, options)
}
pub fn search(mut args: Vec<String>) -> Result<(), String> {
    let options: Options = Options::new();
    args.remove(0);

    macro_handler("search", args, options)
}

// TODO: main optdeps operation

fn macro_handler(operation: &str, args: Vec<String>, options: Options) -> Result<(), String> {
    let mut packages: Vec<String> = vec![];
    for pack in args {
        if pack.starts_with('@') {
            match pack.as_str() {
                "@orphans" => {
                    let orphan_packages: Vec<String> = orphans()?;
                    orphan_packages
                        .iter()
                        .for_each(|x| packages.push(x.to_owned()));
                }
                "@optdeps" => {
                    todo!()
                }
                _ => return Err(String::from("Macro not implemented or invalid")),
            }
        } else {
            packages.push(pack);
        }
    }
    build(operation, packages, options)
}
