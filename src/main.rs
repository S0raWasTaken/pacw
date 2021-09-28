/*
   pacman -Syu -> pacw install (or pacw i)
   pacman -Si -> pacw show (or pacw info)
   pacman -Qqdt -> pacw orphans
   pacman -Rs $(pacman -Qqdt) -> pacw remove @orphans (@ will specify another operation to use)
   pacman -Rs -> pacw remove

   @ usage

*/
use uuid::Uuid;
use std::env::temp_dir;
use std::env;
use std::fs::File;
use std::io::stdin;
use std::io::Write;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;
use std::vec::Vec;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut final_command: String = "sudo pacman".to_owned();
    // TODO: add support to doas via config file and/or autodetection
    let mut main_operation: &str = "";
    let mut packages = String::new();
    let dir = temp_dir();

    let mut i = 0;

    for x in args {
        let arg: &str = x.as_str();
        if i == 0 {
            match arg {
                "install" => {
                    final_command += " -Syu";
                    main_operation = "install";
                }
                "show" => {
                    final_command += " -Si";
                    main_operation = "show";
                }
                "orphans" => {
                    final_command += " -Qqdt";
                    main_operation = "orphans";
                }
                "remove" => {
                    final_command += " -Rs";
                    main_operation = "remove";
                }
                _ => {
                    println!("Operation {} is not known", arg);
                    exit(1);
                }
            }
        } else if arg.starts_with('@') {
            match main_operation {
                "install" => {}
                "remove" => {}
                _ => {
                    println!("The operation {} does not support macros", main_operation);
                    exit(2)
                }
            }
            match arg {
                "@orphans" => {
                    final_command += " $(pacman -Qqdt)";
                },
                // TODO: add more macro operations (optdeps)
                _ => {
                    println!(
                        "Operation \"{}\" unknown or not supported by macro syntax",
                        arg
                    );
                    exit(3);
                }
            }
        } else {
            // TODO: check if package exists and warn user before continuing
            packages += arg;
            packages += " ";
        }
        i = 1;
    }
    for mut package in packages.split(' ') {
        if !package.is_empty() {
            package = package.trim();
            final_command += &(" ".to_string() + package);
        }
    }
    println!("\"{}\"", final_command);
    println!("Is this right?");
    // TODO: Add explanation to the command

    let mut prompt = String::new();
    println!("[Y/N]: ");

    stdin()
        .read_line(&mut prompt)
        .expect("Could not read prompt");

    prompt = prompt.trim().replace("\n", "");

    match prompt.to_ascii_lowercase().as_str() {
        "yes" => {}
        "y" => {}
        "no" => exit(0),
        "n" => exit(0),
        _ => {
            println!("That's not a valid prompt, for security reasons I will exit.");
            exit(4);
        }
    }

    let file_name = format!("{}/{}.pacw-cmd", dir.to_str().unwrap(), Uuid::new_v4());
    let mut file = File::create(&file_name).unwrap();
    write!(file, "{}", final_command).unwrap();
    
    let mut child = Command::new("/bin/bash")
        .arg(file_name)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .unwrap();

    let exit_code = child.wait().unwrap();
    if !exit_code.success() {
        println!("Pacman exit with code non-zero");
        exit(10);
    }
}
