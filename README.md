# PACW
A pacman wrapper and utility tool for simplicity and usability<br/>
Made in rust :)

## Usage:

> [] OPTIONAL, <> REQUIRED
#### Installing, updating and upgrading
```bash
pacw install [package] # -> translates to "pacman -Syu"
```

#### Printing package info
```bash
pacw show <package> # -> same as "pacman -Si"
```

#### Showing orphan packages
```bash
pacw orphans # -> same as "pacman -Qqdt"
```

#### Removing packages
```bash
pacw remove <package> # -> same as "pacman -Rs"
```

#### Using macros
> Note: this feature is still in development, so there's not many things to do yet<br/>

Examples:
```bash
pacw remove @orphans # -> same as "pacman -Rs $(pacman -Qqdt)"
```

## TODO:
- Add support to doas (sudo alternative)
- Add more operations (like optional deps)
- Add more macro operations
- Check package existence before calling the final command
- Add a friendly explanation of what pacw is doing before running the final command
- Organize code in multiple files
- Automate compiling with Make
- Add compiling/instalation instructions
- Actually post a release
