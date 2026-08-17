use anyhow::Result;
use super::{Action, ActionInfo, ActionParseError};

/// Move mouse to absolute position
#[derive(Debug, Clone)]
pub struct MoveTo(i32, i32);

impl ActionInfo for MoveTo {
    const NAME: &str = "move_to";
    const HELP: &str = "Move mouse to absolute position";
    const SIGNATURE: &str = "<x>, <y>";

    fn parse_args(input: &str) -> Result<Self, ActionParseError> {
        if input.is_empty() {
            Err(ActionParseError::NotEnoughArguments)
        } else {
            let mut iter = input.split_ascii_whitespace();

            let x = iter.next()
                .ok_or_else(|| ActionParseError::ArgumentMissing("x".to_string()))?
                .trim();
            let x = x
                .parse::<i32>()
                .map_err(|_| ActionParseError::InvalidValue("x".to_string(), x.to_string()))?;

            let y = iter.next()
                .ok_or_else(|| ActionParseError::ArgumentMissing("y".to_string()))?
                .trim();
            let y = y
                .parse::<i32>()
                .map_err(|_| ActionParseError::InvalidValue("y".to_string(), y.to_string()))?;

            // check for extra arguments
            if iter.next().is_some() {
                return Err(ActionParseError::TooManyArguments);
            }

            Ok(Self(x, y))
        }
    }
}

impl Action for MoveTo {
    fn serialize(&self) -> String {
        Self::NAME.to_string()
    }

    fn execute(&self, app: &mut crate::App) -> Result<()> {
        app.mouse.rel_move(self.0 - app.mouse_position.x, self.1 - app.mouse_position.y)?;

        Ok(())
    }
}
