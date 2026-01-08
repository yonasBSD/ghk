use anyhow::Result;
use crate::{gh, util};
use crate::cli::UserCmd;

pub fn run(cmd: UserCmd) -> Result<()> {
    match cmd {
        UserCmd::List => {
            if !gh::loggedin() {
                util::warn("No accounts found");
                util::dim("Run 'ghk login' to connect your GitHub account");
                return Ok(());
            }
            
            util::ok("Your accounts:");
            gh::listusers()?;
        }
        UserCmd::Switch { name } => {
            if !gh::loggedin() {
                util::warn("Not logged in");
                util::dim("Run 'ghk login' first");
                return Ok(());
            }
            
            util::info(&format!("Switching to {}...", name));
            gh::switchuser(&name)?;
            util::ok(&format!("Now using {}", name));
        }
    }
    Ok(())
}
