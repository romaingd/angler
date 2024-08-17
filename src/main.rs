use std::{
    error::Error,
    fs::OpenOptions,
    io::Write,
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
};

fn main() -> Result<(), Box<dyn Error>> {
    let pattern_half = "fail";
    let rg_cmd = format!(
        "rg -e '{}-{}' ~/code/angler/ && exit 1 || exit 0",
        pattern_half, pattern_half
    );
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
