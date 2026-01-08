use anyhow::{Result, bail};
use crate::{git, util};

pub fn run() -> Result<()> {
    if !git::isrepo() {
        util::err("Not a git repository");
        bail!("Not a git repository");
    }

    let changes = git::haschanges()?;
    if !changes {
        util::ok("No changes");
        return Ok(());
    }

    // Run git diff with color
    std::process::Command::new("git")
        .args(["diff", "--color=always"])
        .status()?;

    println!();
    
    // Show summary
    let files = git::changedfiles()?;
    util::info(&format!("{} file(s) changed", files.len()));
    
    Ok(())
}
