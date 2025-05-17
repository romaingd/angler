use std::path::PathBuf;
use std::process;

use crate::error::AnglerError;

/// Install the angler pre-commit hook in the specified directory
///
/// If no directory is provided, it will automatically detect the git hooks directory.
/// Creates a backup of any existing pre-commit hook.
pub fn install(dir: Option<PathBuf>) -> Result<(), AnglerError> {
    println!("Installing angler pre-commit hook...");

    let hooks_dir = make_hooks_dir(dir)?;

    let pre_commit_path = hooks_dir.join("pre-commit");
    if pre_commit_path.exists() {
        let backup_path = pre_commit_path.with_extension("bak");
        println!("- Backing up existing hook to {}", backup_path.display());
        std::fs::rename(&pre_commit_path, &backup_path).map_err(AnglerError::Io)?;
    }

    write_default_hook(&pre_commit_path)?;

    println!(
        "✓ Successfully installed angler pre-commit hook to {}",
        pre_commit_path.display()
    );

    Ok(())
}

/// Create the hooks directory if necessary.
///
/// If no `dir` is provided, falls back to the default git hooks directory.
fn make_hooks_dir(dir: Option<PathBuf>) -> Result<PathBuf, AnglerError> {
    let hooks_dir = match dir {
        Some(path) => path,
        None => {
            // Default to .git/hooks in the current git repository
            let output = process::Command::new("git")
                .args(["rev-parse", "--git-dir"])
                .output()
                .map_err(|e| AnglerError::Git(format!("Failed to detect git directory: {}", e)))?;

            if !output.status.success() {
                return Err(AnglerError::Git("Not a git repository".to_string()));
            }

            let git_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
            PathBuf::from(git_dir).join("hooks")
        }
    };
    if !hooks_dir.exists() {
        std::fs::create_dir_all(&hooks_dir).map_err(AnglerError::Io)?;
    }
    Ok(hooks_dir)
}

/// Write a default pre-commit hook template to the provided path
fn write_default_hook(pre_commit_path: &PathBuf) -> Result<(), AnglerError> {
    let pre_commit_content = r#"#!/bin/sh
# angler pre-commit hook
# Installed by angler

if ! command -v angler &> /dev/null; then
    echo "Error: angler is not installed or not in PATH"
    echo "Install with cargo install angler"
    exit 1
fi

exec angler run
"#;
    std::fs::write(pre_commit_path, pre_commit_content).map_err(AnglerError::Io)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(pre_commit_path)
            .map_err(AnglerError::Io)?
            .permissions();
        perms.set_mode(0o755); // rwxr-xr-x permissions
        std::fs::set_permissions(pre_commit_path, perms).map_err(AnglerError::Io)?
    };
    Ok(())
}
