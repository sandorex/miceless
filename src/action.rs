#![allow(dead_code)]

mod quit;
mod click;
mod show;
mod center;
mod r#move;
mod r#move_to;
mod sleep;

use std::{error::Error, fmt::{Debug, Display}, rc::Rc};
use anyhow::{Context, Result, anyhow};
use std::{collections::HashMap, sync::LazyLock};

// TODO make an alias to reduce the noise
// pub type ActionRc = Rc<dyn Action>;

#[derive(Debug, Clone)]
pub enum ActionParseError {
    InvalidValue(String, String),
    ArgumentMissing(String),
    NotEnoughArguments,
    TooManyArguments,
    Custom(String),
}

// TODO?
// impl From<std::num::ParseIntError> for ActionParseError {
//     fn from(value: std::num::ParseIntError) -> Self {
//
//     }
// }

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
    /// Serialize the action
    fn serialize(&self) -> String;

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

// TODO proper testing
pub fn parse_action(input: &str) -> Result<Rc<dyn Action>> {
    // take the action name
    let (name, args) = input.trim().split_once(' ').unwrap_or((input, ""));

    let action = ACTIONS_MAP
        .get(name)
        .ok_or_else(|| anyhow!("invalid action {name}"))?;

    action.parse_args(args)
        .with_context(|| anyhow!("error parsing action {name}'s arguments"))
}

pub fn parse_action_list(input: &str) -> Result<Vec<Rc<dyn Action>>> {
    input
        .trim()
        .split(';')
        .map(str::trim)
        .map(parse_action)
        .collect::<Result<Vec<_>>>()
}

// reduces the boilerplate for single action
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

pub use click::Click;
pub use show::Show;
pub use center::Center;
pub use quit::Quit;
pub use r#move::Move;
pub use r#move_to::MoveTo;
pub use sleep::Sleep;

register_actions!(ACTIONS_MAP,
    ClickFactory: Click,
    ShowFactory: Show,
    CenterFactory: Center,
    QuitFactory: Quit,
    MoveFactory: Move,
    MoveToFactory: MoveTo,
    SleepFactory: Sleep,
);
