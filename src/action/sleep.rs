use anyhow::Result;
use super::{Action, ActionInfo, ActionParseError};

/// Move mouse to absolute position
#[derive(Debug, Clone)]
pub struct Sleep(u32);

impl ActionInfo for Sleep {
    const NAME: &str = "sleep";
    const HELP: &str = r#"Sleep for duration in milliseconds.
WARNING: current implementation freezes whole application so it won't respond to close requests either"#;
    const SIGNATURE: &str = "<duration (millis)>";

    fn parse_args(input: &str) -> Result<Self, ActionParseError> {
        if input.is_empty() {
            Err(ActionParseError::NotEnoughArguments)
        } else {
            let mut iter = input.split_ascii_whitespace();

            let sleep = iter.next()
                .ok_or_else(|| ActionParseError::ArgumentMissing("sleep".to_string()))?
                .trim();
            let sleep = sleep
                .parse::<u32>()
                .map_err(|_| ActionParseError::InvalidValue("sleep".to_string(), sleep.to_string()))?;

            // check for extra arguments
            if iter.next().is_some() {
                return Err(ActionParseError::TooManyArguments);
            }

            Ok(Self(sleep))
        }
    }
}

impl Action for Sleep {
    fn serialize(&self) -> String {
        Self::NAME.to_string()
    }

    // TODO this should be replaced by proper timer that does not freeze the application
    fn execute(&self, _app: &mut crate::App) -> Result<()> {
        std::thread::sleep(std::time::Duration::from_millis(self.0 as u64));

        Ok(())
    }
}
