use anyhow::Result;
use super::{Action, ActionInfo, ActionParseError};

/// Hide/show the window
#[derive(Debug, Clone)]
pub struct Show(pub bool);

impl ActionInfo for Show {
    const NAME: &str = "show";
    const HELP: &str = "Show or hide the window";
    const SIGNATURE: &str = "<true|false|0|1>";

    fn parse_args(input: &str) -> Result<Self, ActionParseError> {
        if input.contains(",") {
            return Err(ActionParseError::TooManyArguments);
        }

        match input.to_lowercase().trim() {
            "true" | "1" => Ok(Self(true)),
            "false" | "0" => Ok(Self(false)),
            "" => Err(ActionParseError::NotEnoughArguments),
            _ => Err(ActionParseError::InvalidValue("state".to_string(), input.to_string())),
        }
    }
}

impl Action for Show {
    fn serialize(&self) -> String {
        format!("{} {}", Self::NAME, self.0)
    }

    fn execute(&self, app: &mut crate::App) -> Result<()> {
        if self.0 {
            app.open_window()?;
        } else {
            app.close_window()?;
        }

        Ok(())
    }
}

