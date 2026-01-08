use anyhow::{Result, bail};
use dialoguer::Input;
use crate::{git, gh, util};

pub fn run() -> Result<()> {
    // Check prerequisites
    if !git::isrepo() {
        util::err("Not a git repository");
        util::dim("Run 'ghk init' first");
        bail!("Not a git repository");
    }

    if !gh::loggedin() {
        util::err("Not logged in to GitHub");
        util::dim("Run 'ghk login' first");
        bail!("Not logged in");
    }

    if !git::hasremote() {
        util::err("Not connected to GitHub");
        util::dim("Run 'ghk create' first to create a repository");
        bail!("No remote configured");
    }

    // Check for changes
    let changes = git::haschanges()?;
    if !changes {
        util::ok("Already up to date");
        util::dim("No changes to save");
        return Ok(());
    }

    // Show what will be saved
    util::info("Changes to save:");
    let files = git::changedfiles()?;
    for file in files.iter().take(10) {
        util::dim(&format!("  {}", file));
    }
    if files.len() > 10 {
        util::dim(&format!("  ... and {} more", files.len() - 10));
    }

    // Get commit message
    let msg: String = Input::new()
        .with_prompt("What did you change?")
        .default("Update".to_string())
        .interact_text()?;

    // Stage, commit, push
    util::info("Saving...");
    git::addall()?;
    git::commit(&msg)?;
    git::push()?;

    util::ok("Saved to GitHub!");
    Ok(())
}
