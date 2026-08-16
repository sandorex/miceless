#![allow(dead_code)]

use std::{error::Error, fmt::{Debug, Display}, rc::Rc};
use crate::{mouse::MouseKey, util::{Direction4, Direction8}};
use anyhow::Result;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, Clone)]
pub enum ActionParseError {
    InvalidValue(String, String),
    ArgumentMissing(String),
    NotEnoughArguments,
    TooManyArguments,
    Custom(String),
}

impl Display for ActionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidValue(key, value) => write!(f, "Invalid value {value:?} for {key:?}")?,
            Self::ArgumentMissing(x) => write!(f, "{x:?} argument not specified")?,
            Self::NotEnoughArguments => f.write_str("Not enough arguments provided")?,
            Self::TooManyArguments => f.write_str("Too many arguments provided")?,
            Self::Custom(x) => f.write_str(x)?,
        }

        Ok(())
    }
}

impl Error for ActionParseError {}

/// Define what happens when action is executed
pub trait Action: Debug {
    /// Execute the action
    fn execute(&self, app: &mut crate::App) -> Result<()>;
}

// NOTE this is separated so `Action` remains dyn-compatible
trait ActionInfo: Sized {
    /// Name of the action (ex. MoveTo -> move_to)
    const NAME: &str;

    /// Help message of the action
    const HELP: &str;

    /// Signature of the action
    const SIGNATURE: &str;

    /// Parse input for arguments and construct the action
    fn parse_args(input: &str) -> Result<Self, ActionParseError>;
}

/// Type that parses the action, all part of overcomplicated system so i do not have to register
/// each action by hand
///
/// NOTE: sync is safe as this should never have any state
pub trait ActionFactory: Sync {
    fn name(&self) -> &'static str;
    fn help(&self) -> &'static str;
    fn signature(&self) -> &'static str;

    /// Parse input for arguments and construct the action
    fn parse_args(&self, input: &str) -> Result<Rc<dyn Action>, ActionParseError>;
}

macro_rules! register_actions {
    ($map:ident, $($name_factory:ident: $name:ident ,)+) => {
        $(
            struct $name_factory;
            impl ActionFactory for $name_factory {
                fn name(&self) -> &'static str {
                    $name::NAME
                }

                fn help(&self) -> &'static str {
                    $name::HELP
                }

                fn signature(&self) -> &'static str {
                    $name::SIGNATURE
                }

                fn parse_args(&self, input: &str) -> Result<Rc<dyn Action>, ActionParseError> {
                    Ok(Rc::new($name::parse_args(input)?))
                }
            }
        )+

        pub static $map: LazyLock<HashMap<&'static str, &'static dyn ActionFactory>> = LazyLock::new(|| HashMap::from([
            $(
                ($name_factory.name(), &$name_factory as &dyn ActionFactory),
            )+
        ]));
    }
}

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
            let mut iter = input.splitn(3, ',');

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
    fn execute(&self, app: &mut crate::App) -> Result<()> {
        app.mouse.click(self.0, self.1.map(|x| std::time::Duration::from_millis(x as u64)))?;
        Ok(())
    }
}

/// Hide/show the window
#[derive(Debug, Clone)]
pub struct Window(pub bool);

impl ActionInfo for Window {
    const NAME: &str = "Hide";
    const HELP: &str = "Hides or shows the window";
    const SIGNATURE: &str = "<Bool>";

    fn parse_args(input: &str) -> Result<Self, ActionParseError> {
        if input.contains(",") {
            return Err(ActionParseError::TooManyArguments);
        }

        match input.to_lowercase().trim() {
            "true" | "1" | "shown" => Ok(Self(true)),
            "false" | "0" | "hidden" => Ok(Self(false)),
            "" => Err(ActionParseError::NotEnoughArguments),
            _ => Err(ActionParseError::InvalidValue("state".to_string(), input.to_string())),
        }
    }
}

// TODO i need to recreate the window or it just spawns at weird locations, so split the window into
// its own struct and create a new one each time
//
// currently i cannot kill the window as there are references to many things
impl Action for Window {
    fn execute(&self, app: &mut crate::App) -> Result<()> {
        if self.0 {
            app.canvas.window_mut().show();
        } else {
            app.canvas.window_mut().hide();
        }

        Ok(())
    }
}

register_actions!(ACTIONS_MAP,
    ClickFactory: Click,
    WindowFactory: Window,
);

// TODO
pub fn parse_action(input: &str) -> Result<Rc<dyn Action>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actions() {
        println!("{:?}", Click::parse_args("left,").unwrap());
        assert_eq!(1, 2);
    }
}
