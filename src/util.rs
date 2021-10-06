use std::process::{Command, Output};

pub fn get_optdeps(package: String) -> Result<Vec<String>, String> {
    let child: Output = Command::new("/bin/pacman")
        .args(["-Si", package.as_str()])
        .output()
        .map_err(|_| String::from("Couldn't get info from package"))?;

    if !child.status.success() {
        return Err(format!("Package '{}' does not exist", package));
    }

    let optional_deps = String::from_utf8(child.stdout)
        .map_err(|_| String::from("uhhh, something went wrong here"))?
        .split("Optional Deps")
        .map(|s| String::from(s))
        .collect::<Vec<String>>()[1]
        .as_str()
        .trim()
        .strip_prefix(": ")
        .iter()
        .map(|x| x.to_owned())
        .collect::<Vec<&str>>()[0]
        .to_string()
        .split("Conflicts")
        .map(|x| String::from(x))
        .collect::<Vec<String>>()[0]
        .as_str()
        .trim()
        .to_owned()
        .split('\n')
        .map(|x| String::from(
            x.trim()
                .split(':')
                .collect::<Vec<&str>>()[0]
        ))
        .collect::<Vec<String>>();

    Ok(optional_deps)
}
