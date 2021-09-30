use std::process::{Command, Output};

pub fn orphans() -> Result<Vec<String>, String> {
    let child: Output = Command::new("/bin/pacman")
        .arg("-Qqdt")
        .output()
        .map_err(|_| String::from("Couldn't run \"pacman -Qqdt\""))?;

    let orphan_packages: Vec<String> = String::from_utf8(child.stdout)
        .map_err(|_| String::from("a"))?
        .trim()
        .split('\n')
        .map(|x| x.to_owned())
        .collect();
    Ok(orphan_packages)
}

// TODO: macro optdeps operation
