#![allow(dead_code)]

use super::{Action, ActionInfo, ActionFactory, ActionParseError};
use std::rc::Rc;
use anyhow::Result;
use std::{collections::HashMap, sync::LazyLock};

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

pub use super::quit::Quit;
pub use super::click::Click;
pub use super::show::Show;
pub use super::center::Center;

register_actions!(ACTIONS_MAP,
    ClickFactory: Click,
    ShowFactory: Show,
    CenterFactory: Center,
    QuitFactory: Quit,
);
