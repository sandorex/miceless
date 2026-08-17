use anyhow::Result;
use super::{Action, ActionInfo, ActionParseError};

/// Move mouse by amount (relative)
#[derive(Debug, Clone)]
pub struct Move(i32, i32);

impl ActionInfo for Move {
    const NAME: &str = "move";
    const HELP: &str = "Move mouse by an amount";
    const SIGNATURE: &str = "<x>, <y>";

    fn parse_args(input: &str) -> Result<Self, ActionParseError> {
        if input.is_empty() {
            Err(ActionParseError::NotEnoughArguments)
        } else {
            let mut iter = input.splitn(3, ',');

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

impl Action for Move {
    fn serialize(&self) -> String {
        Self::NAME.to_string()
    }

    fn execute(&self, app: &mut crate::App) -> Result<()> {
        app.mouse.rel_move(self.0, self.1)?;

        Ok(())
    }
}
