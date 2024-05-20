use std::{ffi::OsString, os::unix::ffi::OsStringExt, path::PathBuf, process::Command};

use crate::change_id;

/// Get Command output, stripping trailing newlines
fn trimmed_stdout(mut cmd: Command) -> Result<String, std::io::Error> {
    let output = cmd.output()?;
    Ok(OsString::from_vec(output.stdout)
        .to_string_lossy()
        .trim_end_matches('\n')
        .to_owned())
}

pub fn get_workspace_root() -> Result<PathBuf, std::io::Error> {
    let mut cmd = Command::new("jj");
    cmd.args(["workspace", "root"]);
    let workspace_root = trimmed_stdout(cmd)?;
    Ok(PathBuf::from(workspace_root))
}

pub fn get_current_change_id() -> Result<String, std::io::Error> {
    //! jj log -T 'change_id' -l 1 --no-graph -r @
    //! This will print the change id, and only the change id, of the working copy
    let mut cmd = Command::new("jj");
    cmd.args(["log", "-T", "change_id", "-l", "1", "--no-graph", "-r", "@"]);
    let output = trimmed_stdout(cmd)?;
    Ok(output)
}

pub fn get_parent_change_id() -> Result<String, std::io::Error> {
    let mut cmd = Command::new("jj");
    cmd.args([
        "log",
        "-T",
        "change_id",
        "-l",
        "1",
        "--no-graph",
        "-r",
        "@-",
    ]);
    let output = trimmed_stdout(cmd)?;
    Ok(output)
}

/// From the JJ source code:
/// > A commit is discardable if it has one parent, no change from its
/// > parent, and an empty description.
/// If we switch away from a "discardable" commit, JJ will automatically abandon that commit.
/// So, to switch back we need to instead remember the parent commit.
pub fn get_is_discardable() -> Result<bool, std::io::Error> {
    let mut cmd = Command::new("jj");
    cmd.args([
        "log",
        "-T",
        "description",
        "-l",
        "1",
        "--no-graph",
        "-r",
        "@",
    ]);
    let description = trimmed_stdout(cmd)?;

    let mut cmd = Command::new("jj");
    cmd.args([
        "log",
        "-T",
        r#"parents.map(|p| p.change_id().short()).join("\n")"#,
        "-l",
        "1",
        "--no-graph",
        "-r",
        "@",
    ]);
    let parent_change_ids = trimmed_stdout(cmd)?;
    let parent_change_ids: Vec<String> = parent_change_ids
        .split('\n')
        .map(|s| s.to_owned())
        .collect();

    let mut cmd = Command::new("jj");
    cmd.args(["log", "-T", "empty", "-l", "1", "--no-graph", "-r", "@"]);
    let is_no_changes = trimmed_stdout(cmd)?;
    let is_no_changes = (match is_no_changes.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "jj did not return expected 'true' or 'false'",
        )),
    })?;

    Ok(description.len() == 0 && parent_change_ids.len() <= 1 && is_no_changes)
}

pub fn edit_change(change_id: String) -> Result<(), std::io::Error> {
    let mut cmd = Command::new("jj");
    cmd.args(["edit", &change_id]);
    let output = cmd.output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "'jj edit' terminated with non-zero exit code",
        ))
    }
}

pub fn new_from_change(change_id: String) -> Result<(), std::io::Error> {
    let mut cmd = Command::new("jj");
    cmd.args(["new", &change_id]);
    let output = cmd.output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "'jj new' terminated with non-zero exit code",
        ))
    }
}
