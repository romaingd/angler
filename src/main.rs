use std::{
    error::Error,
    fs::OpenOptions,
    io::Write,
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
};

fn main() -> Result<(), Box<dyn Error>> {
    let rg_cmd = "rg -e 'fail-pat' ~/code/angler/";
    let hook_path = "/Users/romain/code/angler/.git/hooks/pre-commit";

    let mut options = OpenOptions::new();
    let mut file = options
        .write(true)
        .create(true)
        .mode(0o777)
        .open(hook_path)
        .expect("Could not open file");

    let mut perms = file.metadata()?.permissions();
    perms.set_mode(0o777);
    file.set_permissions(perms)?;

    file.write_all(rg_cmd.as_bytes())
        .expect("Could not write to file");

    Ok(())
}
