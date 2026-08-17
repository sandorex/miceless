use super::{Action, ActionInfo, ActionParseError};
use anyhow::Result;

/// Quit the application
#[derive(Debug, Clone)]
pub struct Quit;

impl ActionInfo for Quit {
    const NAME: &str = "quit";
    const HELP: &str = "Quit the application";
    const SIGNATURE: &str = "";

    fn parse_args(input: &str) -> Result<Self, ActionParseError> {
        if !input.is_empty() {
            return Err(ActionParseError::TooManyArguments);
        }

        Ok(Self)
    }
}

impl Action for Quit {
    fn serialize(&self) -> String {
        Self::NAME.to_string()
    }

    fn execute(&self, app: &mut crate::App) -> Result<()> {
        app.running = false;

        Ok(())
    }
}
