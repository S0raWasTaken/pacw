use crate::command_builder::build;
use crate::macro_operations::orphans;
use crate::util::get_optdeps;
use crate::Options;

pub fn install(mut args: Vec<String>) -> Result<(), String> {
    let mut options: Options = Options::new();
    options.sudo = true;
    options.colors = false;
    args.remove(0);

    macro_handler("install", args, options).map(|_| ())
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

    macro_handler("remove", args, options).map(|_| ())
}
pub fn show(mut args: Vec<String>) -> Result<(), String> {
    let options: Options = Options::new();
    args.remove(0);

    macro_handler("show", args, options).map(|_| ())
}
pub fn search(mut args: Vec<String>) -> Result<(), String> {
    let options: Options = Options::new();
    args.remove(0);

    macro_handler("search", args, options).map(|_| ())
}

pub fn optdeps(mut args: Vec<String>) -> Result<(), String> {
    let mut options: Options = Options::new();
    options.nobuild = true;
    args.remove(0);

    let mut opt_deps: Vec<String> = Vec::new();
    let packages = macro_handler("optdeps", args, options)?;
    for package in packages {
        let deps_from_pkg = get_optdeps(package)?;
        deps_from_pkg
            .iter()
            .for_each(|dep| opt_deps.push(dep.to_owned()));
    }

    Ok(println!("{:?}", opt_deps))
}

// TODO: main optdeps operation

fn macro_handler(operation: &str, args: Vec<String>, options: Options) -> Result<Vec<String>, String> {
    let mut packages: Vec<String> = vec![];
    for pack in &args {
        if pack.starts_with('@') {
            match pack.as_str() {
                "@orphans" => {
                    let orphan_packages: Vec<String> = orphans()?;
                    orphan_packages
                        .iter()
                        .for_each(|x| packages.push(x.to_owned()));
                }
                "@optdeps" => {
                    let mut opt = Options::new();
                    opt.nobuild = true;
                    let fullcmd = args.join(" ")
                        .split_once("@optdeps")
                        .map(|x| String::from(x.1))
                        .iter()
                        .map(|x| x.to_owned())
                        .collect::<Vec<String>>()[0]
                        .as_str()
                        .trim()
                        .to_owned();
                    let opt_args: Vec<String> = fullcmd.split(' ')
                        .map(|x| String::from(x))
                        .collect();
                    let mut opt_deps: Vec<String> = vec![];

                    for package in &opt_args {
                        get_optdeps(package.to_string())?.iter().for_each(|x| {
                            opt_deps.push(x.to_string());
                        });
                    }
                    opt_deps.iter().for_each(|x| packages.push(x.to_string()));
                    opt_args.iter().for_each(|x| packages.push(x.to_string()));
                    break;
                }
                _ => return Err(String::from("Macro not implemented or invalid")),
            }
        } else {
            packages.push(pack.to_string());
        }
    }
    if !options.nobuild {
        build(operation, packages, options).map(|_| vec![String::from("")])
    } else {
        Ok(packages)
    }
}
