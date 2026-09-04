use clap::{Args, Parser, Subcommand};

/// Creates a fake mouse that you can control using keybindings, meant as a
/// way to click things without reaching for a mouse when its something simple
///
/// Like mouseless but with mice instead!
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<CliCommands>,
}

// TODO mode where each key pressed is printed so configuration is easier to do
#[derive(Subcommand, Debug, Clone)]
pub enum CliCommands {
    /// Creates fake mouse so you can configure it in the desktop environment
    Configure,

    /// Runs sequence of actions without the window
    ///
    /// Note that you can use `,` in place of ';'
    Script(CmdScript),

    /// Lists all actions with information about them
    Actions,
}

#[derive(Args, Debug, Clone)]
pub struct CmdScript {
    /// Actions to execute in sequence
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
    pub action_list: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
