#![allow(dead_code)]

use std::fmt::Debug;
use anyhow::Result;
use std::str::FromStr;
use crate::{App, mouse::MouseKey};
use clap::{Args, CommandFactory, Parser, Subcommand};

// TODO do i need serialization of actions?

#[derive(Parser, Debug, Clone)]
#[command(disable_help_flag = true, disable_help_subcommand = true)]
struct ActionClap {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
#[command(rename_all = "snake_case", disable_help_flag = true, disable_help_subcommand = true)]
pub enum Action {
    /// Press a mouse button and release it after a delay, essentially a click
    Click(ActionClick),

    /// Press a mouse button
    Press(ActionPress),

    /// Release a mouse button
    Release(ActionPress),

    /// Show the window
    Show,

    /// Hide the window
    Hide,

    /// Close the application
    Quit,

    /// Move mouse to the center of the grid
    MoveToCenter,

    /// Move the mouse
    Move(ActionMove),

    /// Move the mouse to an absolute position on screen
    MoveTo(ActionMove),

    /// Scroll by an amount
    Scroll(ActionMove),

    /// Sleep for a duration (currently freezes the whole application)
    Sleep(ActionSleep),

    // TODO
    // /// Resets the rectangle
    // Reset,

    /// Pick a part of the window split into a grid
    Grid(ActionGrid),
}

#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct ActionClick {
    pub key: MouseKey,

    #[clap(default_value = "20", value_name = "DELAY_MS")]
    pub delay: u32,
}

#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct ActionPress {
    pub key: MouseKey,
}

#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct ActionShow {
    /// Whether to show the window
    #[clap(action = clap::ArgAction::Set, value_name = "TRUE|FALSE")]
    pub show: bool,
}

#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct ActionMove {
    #[arg(allow_hyphen_values = true)]
    pub x: i32,

    #[arg(allow_hyphen_values = true)]
    pub y: i32,
}

#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct ActionSleep {
    #[clap(value_name = "DURATION_MS")]
    pub duration: u16,
}

#[derive(Args, Debug, Clone, PartialEq, Eq)]
pub struct ActionGrid {
    /// Width of the grid
    pub w: u16,

    /// Height of the grid
    pub h: u16,

    /// Index of the cell horizontally
    pub x: u16,

    /// Index of the cell vertically
    pub y: u16,
}

impl Action {
    /// Returns list of all actions with their usage and help
    pub fn get_actions() -> Vec<(String, String, String)> {
        let mut cmd = ActionClap::command();
        let mut output: Vec<(String, String, String)> = vec![];

        for subcmd in cmd.get_subcommands_mut() {
            let name = subcmd.get_name().to_string();
            let usage = subcmd
                .render_usage()
                .to_string()
                .strip_prefix(&format!("Usage: {name}")) // remove the extra text
                .unwrap()
                .trim()
                .to_string();

            // get only the actual help
            let help = subcmd
                .render_long_help()
                .to_string()
                .split("Usage:")
                .next()
                .unwrap()
                .trim()
                .to_string();

            output.push((name, usage, help));
        }

        output
    }

    pub fn execute(&self, app: &mut App) -> anyhow::Result<()> {
        use std::time::Duration;

        match self {
            Self::Click(ActionClick { key, delay }) => app.mouse.click(*key, Some(Duration::from_millis(*delay as u64)))?,
            Self::Press(ActionPress { key }) => app.mouse.press(key)?,
            Self::Release(ActionPress { key }) => app.mouse.release(key)?,
            Self::Show => app.open_window()?,
            Self::Hide => app.close_window()?,
            Self::Quit => app.running = false,
            Self::Move(ActionMove { x, y }) => app.mouse.rel_move(*x, *y)?,
            Self::MoveTo(ActionMove { x, y }) => {
                app.mouse.reset_position()?;
                app.mouse.rel_move(*x, *y)?;
            }
            Self::Scroll(ActionMove { x, y }) => app.mouse.scroll(*x, *y)?,
            Self::Sleep(ActionSleep { duration }) => std::thread::sleep(Duration::from_millis(*duration as u64)),
            Self::Grid(ActionGrid { w, h, x, y }) => {
                app.rect = app.rect.divide(
                    Into::<i32>::into(*w),
                    Into::<i32>::into(*h),
                    Into::<i32>::into(*x),
                    Into::<i32>::into(*y)
                );
                app.update_requested = true;
            }
            Self::MoveToCenter => {
                let center = app.rect.center();
                app.mouse.rel_move(center.x - app.mouse_position.x, center.y - app.mouse_position.y)?;
            }
        }

        Ok(())
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // the "x" is the command first argument which clap expects, $0 in bash for example
        match ActionClap::try_parse_from(std::iter::once("x").chain(s.split_ascii_whitespace())) {
            Ok(x) => Ok(x.action),
            Err(err) => Err(
                format!("{err}") // remove usage from error message
                    .split("Usage:")
                    .next()
                    .unwrap()
                    .trim()
                    .to_string()
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionList(pub Vec<Action>);

impl FromStr for ActionList {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(";")
            .map(|x| x.trim().parse::<Action>())
            .collect::<Result<Vec<_>, _>>()
            .map(|x| ActionList(x))
    }
}
