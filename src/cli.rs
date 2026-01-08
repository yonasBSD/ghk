use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

#[derive(Parser)]
#[command(name = "ghk")]
#[command(about = "Simple GitHub helper - push code without the complexity")]
#[command(version)]
pub struct Cli {
    /// Suppress output (errors still shown)
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub nocolor: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check and install requirements
    Setup,
    
    /// Start tracking this folder
    Init,
    
    /// Connect to GitHub
    Login,
    
    /// Disconnect from GitHub  
    Logout,
    
    /// Manage GitHub accounts
    #[command(subcommand)]
    User(UserCmd),
    
    /// Create a repository on GitHub
    Create,
    
    /// Save changes to GitHub
    Push,

    /// Alias for push
    #[command(hide = true)]
    Save,
    
    /// Download changes from GitHub
    Pull,

    /// Alias for pull
    #[command(hide = true)]
    Sync,
    
    /// Download a repository
    Clone {
        /// Repository (owner/name or URL)
        repo: Option<String>,
        /// Directory to clone into
        dir: Option<String>,
    },

    /// Alias for clone
    #[command(hide = true)]
    Download {
        repo: Option<String>,
        dir: Option<String>,
    },
    
    /// Show current status
    Status,

    /// Preview changes before saving
    Diff,
    
    /// Undo last commit (keeps changes)
    Undo,
    
    /// Show recent saves
    History {
        /// Number of commits to show
        #[arg(default_value = "10")]
        count: Option<usize>,
    },

    /// Alias for history
    #[command(hide = true)]
    Log {
        #[arg(default_value = "10")]
        count: Option<usize>,
    },
    
    /// Open repository in browser
    Open,

    /// View or edit settings
    Config {
        /// Setting to view/edit
        key: Option<String>,
        /// New value
        value: Option<String>,
    },

    /// Add .gitignore template
    Ignore {
        /// Template name (node, python, rust, go, etc)
        template: Option<String>,
    },

    /// Add a license file
    License {
        /// License type
        #[arg(value_enum)]
        kind: Option<LicenseKind>,
    },

    /// List or switch branches
    Branch {
        /// Branch to switch to
        name: Option<String>,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand)]
pub enum UserCmd {
    /// Show logged in accounts
    List,
    
    /// Switch to a different account
    Switch { 
        /// GitHub username to switch to
        name: String 
    },
}

#[derive(Clone, ValueEnum)]
pub enum LicenseKind {
    Mit,
    Apache,
    Gpl,
    Unlicense,
}
