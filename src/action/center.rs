use anyhow::Result;
use super::{Action, ActionInfo, ActionParseError};

/// Center the mouse to rect
#[derive(Debug, Clone)]
pub struct Center;

impl ActionInfo for Center {
    const NAME: &str = "center";
    const HELP: &str = "Centers the mouse at current rect center";
    const SIGNATURE: &str = "";

    fn parse_args(input: &str) -> Result<Self, ActionParseError> {
        if !input.is_empty() {
            return Err(ActionParseError::TooManyArguments);
        }

        Ok(Self)
    }
}

impl Action for Center {
    fn serialize(&self) -> String {
        Self::NAME.to_string()
    }

    fn execute(&self, app: &mut crate::App) -> Result<()> {
        // TODO maybe i should just use the -1,-1 reset method
        let center = app.rect.center();
        app.mouse.rel_move(center.x - app.mouse_position.x, center.y - app.mouse_position.y)?;
        Ok(())
    }
}
