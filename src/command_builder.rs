use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::{env::temp_dir, fs::File};

use crate::Options;
use uuid::Uuid;

pub fn build(operation: &str, packages: Vec<String>, options: Options) -> Result<(), String> {
    let mut final_command = String::new();
    if options.sudo {
        final_command += "sudo ";
    }
    final_command += "pacman ";

    if options.colors {
        final_command += "--color=auto ";
    }

    match operation {
        "install" => {
            final_command += "-S ";
            final_command += packages.join(" ").as_str();
        }
        "upgrade" => {
            final_command += "-Syu ";
            final_command += packages.join(" ").as_str();
        }
        "remove" => {
            final_command += "-Rs ";
            final_command += packages.join(" ").as_str();
        }
        "show" => {
            final_command += "-Si ";
            final_command += packages.join(" ").as_str();
        }
        "search" => {
            final_command += "-Ss ";
            final_command += packages.join(" ").as_str();
        }
        _ => {}
    }
    //println!("{}", final_command);
    let file_name = format!("{}/{}.pacw-cmd", temp_dir().display(), Uuid::new_v4());

    let mut temp_file =
        File::create(&file_name).map_err(|_| String::from("Could not create tempfile"))?;

    if let Ok(rs) = write!(temp_file, "{}", final_command) {
        rs
    } else {
        return Err(String::from("Could not write to tempfile"));
    };

    let mut cmd = Command::new("/bin/bash");

    if let Ok(mut child) = cmd
        .arg(file_name)
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        match child.try_wait() {
            Ok(Some(_)) => Ok(()),
            Ok(None) => {
                let res = child.wait().map_err(|_| String::from(""))?;
                if !res.success() {
                    //eprintln!("Pacman exit with status code != 0");
                    Err(String::from("Pacman exit with status code != 0"))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err(String::from("Command could not spawn"))
    }
}
