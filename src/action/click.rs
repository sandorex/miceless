use crate::mouse::MouseKey;
use anyhow::Result;
use super::{Action, ActionInfo, ActionParseError};

/// Click with optional delay between clicks in millis
#[derive(Debug, Clone)]
pub struct Click(pub MouseKey, pub Option<u32>);

impl ActionInfo for Click {
    const NAME: &str = "click";
    const HELP: &str = "Presses and releases mouse button after a delay in milliseconds";
    const SIGNATURE: &str = "<MouseKey>[, <Delay>]";

    fn parse_args(input: &str) -> Result<Self, ActionParseError> {
        if input.is_empty() {
            Err(ActionParseError::NotEnoughArguments)
        } else {
            let mut iter = input.split_ascii_whitespace();

            let key = iter.next()
                .ok_or_else(|| ActionParseError::ArgumentMissing("key".to_string()))?
                .trim()
                .parse::<MouseKey>()
                .map_err(|err| ActionParseError::InvalidValue("key".to_string(), err))?;

            let delay = if let Some(delay_raw) = iter.next() {
                Some(
                    delay_raw
                        .trim()
                        .parse::<u32>()
                        .map_err(|_| ActionParseError::InvalidValue("delay".to_string(), delay_raw.to_string()))?
                )
            } else {
                None
            };

            // check for extra arguments
            if delay.is_some() && iter.next().is_some() {
                return Err(ActionParseError::TooManyArguments);
            }

            Ok(Self(key, delay))
        }
    }
}

impl Action for Click {
    fn serialize(&self) -> String {
        use std::fmt::Write;

        let mut output = String::new();

        // SAFETY: writing to string cannot fail

        write!(&mut output, "{} {}", Self::NAME, self.0).unwrap();

        if let Some(delay) = self.1 {
            write!(&mut output, ", {}", delay).unwrap();
        }

        output
    }

    fn execute(&self, app: &mut crate::App) -> Result<()> {
        app.mouse.click(self.0, self.1.map(|x| std::time::Duration::from_millis(x as u64)))?;
        Ok(())
    }
}
