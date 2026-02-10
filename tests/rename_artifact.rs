use std::{error::Error, process::Command};

#[test]
fn rename_artifact() -> Result<(), Box<dyn Error>> {
    let status = Command::new("cargo")
        .args(&["build", "-vv", "--package=examples-rename-artifact"])
        .status()?;
    if !status.success() {
        return Err(format!("examples-rename-artifact failed to build: {}", status).into());
    }

    // TODO

    Ok(())
}
