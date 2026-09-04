mod util;
mod mouse;
mod cli;
mod assets;
mod action;

use action::Action;
use clap::Parser;
use util::*;
use anyhow::{anyhow, Result};
use sdl3::{event::Event, keyboard::{Keycode, Mod}, pixels::Color, render::{TextureQuery, WindowCanvas}, video::WindowFlags};
use std::{collections::HashMap, hash::Hash, str::FromStr, sync::Arc, time::Duration};
use crate::{action::ActionList, mouse::{FakeMouse, MouseKey}};
use bitflags::bitflags;

// TODO write an enum for Keycode that converts to/from sdl keycode

// NOTE im doing this as i may replace SDL in the future
bitflags! {
    /// Represents key modifiers
    #[derive(Debug, Clone, Copy, Eq, Hash)]
    pub struct KeyMod: u16 {
        const LCTRL     = 1;
        const RCTRL     = 1 << 1;
        const CTRL      = 1 | 1 << 1;

        const LSHIFT    = 1 << 2;
        const RSHIFT    = 1 << 3;
        const SHIFT     = 1 << 2 | 1 << 3;

        const LALT      = 1 << 4;
        const RALT      = 1 << 5;
        const ALT       = 1 << 4 | 1 << 5;

        const LMETA     = 1 << 6;
        const RMETA     = 1 << 7;
        const META      = 1 << 6 | 1 << 7;
    }
}

impl From<sdl3::keyboard::Mod> for KeyMod {
    fn from(value: sdl3::keyboard::Mod) -> Self {
        use sdl3::keyboard::Mod;

        let mut output = Self::empty();

        output.set(Self::LCTRL, value.contains(Mod::LCTRLMOD));
        output.set(Self::RCTRL, value.contains(Mod::RCTRLMOD));
        output.set(Self::LSHIFT, value.contains(Mod::LSHIFTMOD));
        output.set(Self::RSHIFT, value.contains(Mod::RSHIFTMOD));
        output.set(Self::LALT, value.contains(Mod::LALTMOD));
        output.set(Self::RALT, value.contains(Mod::RALTMOD));
        output.set(Self::LMETA, value.contains(Mod::LGUIMOD));
        output.set(Self::RMETA, value.contains(Mod::RGUIMOD));

        output
    }
}

// TODO proper tests
impl PartialEq for KeyMod {
    fn eq(&self, other: &Self) -> bool {
        let cmp = |flag| -> bool {
            // NOTE check if either contains the flag for both LEFT and RIGHT variant
            self.contains(flag) == other.intersects(flag)
                || other.contains(flag) == self.intersects(flag)
                || self.intersection(flag).bits() == other.intersection(flag).bits()
        };

        cmp(Self::CTRL)
            && cmp(Self::SHIFT)
            && cmp(Self::ALT)
            && cmp(Self::META)
    }
}

#[cfg(test)]
mod tests {
    use super::KeyMod;

    #[test]
    fn test_keymod_eq() {
        assert_eq!(KeyMod::CTRL, KeyMod::LCTRL);
        assert_eq!(KeyMod::CTRL, KeyMod::RCTRL);
        assert_eq!(KeyMod::CTRL, KeyMod::LCTRL);
        assert_eq!(KeyMod::CTRL, KeyMod::RCTRL);
        assert_eq!(KeyMod::LCTRL, KeyMod::LCTRL);
        assert_ne!(KeyMod::LCTRL, KeyMod::RCTRL);

        assert_eq!(KeyMod::SHIFT, KeyMod::LSHIFT);
        assert_eq!(KeyMod::SHIFT, KeyMod::RSHIFT);
        assert_eq!(KeyMod::LSHIFT, KeyMod::LSHIFT);
        assert_ne!(KeyMod::LSHIFT, KeyMod::RSHIFT);

        assert_eq!(KeyMod::ALT, KeyMod::LALT);
        assert_eq!(KeyMod::ALT, KeyMod::RALT);
        assert_eq!(KeyMod::LALT, KeyMod::LALT);
        assert_ne!(KeyMod::LALT, KeyMod::RALT);

        assert_eq!(KeyMod::META, KeyMod::LMETA);
        assert_eq!(KeyMod::META, KeyMod::RMETA);
        assert_eq!(KeyMod::LMETA, KeyMod::LMETA);
        assert_ne!(KeyMod::LMETA, KeyMod::RMETA);

        assert_eq!(KeyMod::LSHIFT | KeyMod::ALT, KeyMod::LSHIFT | KeyMod::RALT);
        assert_eq!(KeyMod::LSHIFT , KeyMod::SHIFT);
        assert_eq!(KeyMod::RSHIFT , KeyMod::SHIFT);
        assert_eq!(KeyMod::SHIFT , KeyMod::SHIFT);
        assert_eq!(KeyMod::SHIFT , KeyMod::LSHIFT);
        assert_eq!(KeyMod::SHIFT , KeyMod::RSHIFT);
    }
}

/// Combines SDL3 `Keycode` and `Mod` into one hashable struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keybinding {
    pub key: Keycode,
    pub modifiers: KeyMod,
}

// TODO write proper tests
impl FromStr for Keybinding {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts: Vec<_> = s.split('-').collect();
        let key = parts.pop().unwrap();

        let mut modifiers = KeyMod::empty();
        for mod_name in &parts {
            match mod_name.to_lowercase().as_str() {
                "lctrl" => modifiers.insert(KeyMod::LCTRL),
                "rctrl" => modifiers.insert(KeyMod::RCTRL),
                "ctrl" | "c" => modifiers.insert(KeyMod::CTRL),

                "lshift" => modifiers.insert(KeyMod::LSHIFT),
                "rshift" => modifiers.insert(KeyMod::RSHIFT),
                "shift" | "s" => modifiers.insert(KeyMod::SHIFT),

                "lalt" => modifiers.insert(KeyMod::LALT),
                "ralt" => modifiers.insert(KeyMod::RALT),
                "alt" | "a" => modifiers.insert(KeyMod::ALT),

                "lmeta" => modifiers.insert(KeyMod::LMETA),
                "rmeta" => modifiers.insert(KeyMod::RMETA),
                "meta" | "m" => modifiers.insert(KeyMod::META),

                _ => return Err(format!("Invalid key modifiers {s:?}")),
            }
        }

        let keycode = Keycode::from_name(key)
            .ok_or_else(|| format!("Invalid keycode {s:?}"))?;

        Ok(Keybinding {
            key: keycode,
            modifiers: modifiers.into(),
        })
    }
}

// // TODO make sure keybindings with specific modifier and general modifiers are the same
// // LSHIFT == SHIFT == RSHIFT etc
// impl PartialEq for Keybinding {
//     fn eq(&self, other: &Self) -> bool {
//         self.key == other.key && (self.modifiers == other.modifiers)
//     }
// }

// impl Eq for Keybinding {}

impl Hash for Keybinding {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.modifiers.bits().hash(state);
    }
}

pub struct Window {
    pub canvas: WindowCanvas,
}

impl Window {
    pub fn new(video_subsystem: &sdl3::VideoSubsystem) -> Result<Self> {
        let window = video_subsystem.window("Miceless", 800, 600)
            .set_flags(WindowFlags::FULLSCREEN | WindowFlags::NOT_FOCUSABLE | WindowFlags::BORDERLESS | WindowFlags::ALWAYS_ON_TOP | WindowFlags::UTILITY | WindowFlags::TRANSPARENT)
            .build()
            .unwrap();

        // TODO display is always the same as get_primary_display
        // let display = window.get_display()?;
        // let rect = Rect::from(display.get_bounds()?);

        let mut canvas = window.into_canvas();
        let texture_creator = canvas.texture_creator();

        Ok(Self {
            canvas,
        })
    }
}

pub struct App<'a> {
    pub sdl_context: sdl3::Sdl,
    pub ttf_context: sdl3::ttf::Sdl3TtfContext,
    pub video_subsystem: sdl3::VideoSubsystem,
    pub font: sdl3::ttf::Font<'a>,
    pub window: Option<Window>,

    /// Size of the display
    pub full_rect: Rect,

    /// Currently focused rect
    pub rect: Rect,

    pub mouse: FakeMouse,
    pub mouse_position: Point,
    pub running: bool,
    pub keybindings: HashMap<Keybinding, ActionList>,
    pub update_requested: bool,
}

impl<'a> App<'a> {
    pub fn new(keybindings: HashMap<Keybinding, ActionList>) -> Result<Self> {
        let sdl_context = sdl3::init().unwrap();
        let ttf_context = sdl3::ttf::init()?;
        let video_subsystem = sdl_context.video().unwrap();

        // TODO font size
        // SAFETY: it cannot fail as FONT is static
        let font = ttf_context.load_font_from_iostream(sdl3::iostream::IOStream::from_bytes(assets::FONT).unwrap(), 24.0)?;

        let mouse = FakeMouse::new()?;

        // TODO i do not know if this gets proper size of the monitor as both my monitors are same
        // resolution
        let output_size = video_subsystem.get_primary_display()?.get_bounds()?;
        let full_rect = Rect::new(0, 0, output_size.w, output_size.h);

        let app = Self {
            sdl_context,
            ttf_context,
            video_subsystem,
            font,
            mouse,
            full_rect,
            rect: full_rect,
            window: None,
            mouse_position: Default::default(),
            running: true,
            keybindings,
            update_requested: false,
        };

        Ok(app)
    }

    pub fn close_window(&mut self) -> Result<()> {
        drop(self.window.take());

        Ok(())
    }

    /// Open window
    pub fn open_window(&mut self) -> Result<()> {
        self.window = Some(Window::new(&self.video_subsystem)?);

        Ok(())
    }

    pub fn main_loop(&mut self) -> Result<()> {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        while self.running {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Window { win_event, .. } => {
                        match win_event {
                            // draw when the window is shown
                            sdl3::event::WindowEvent::Shown => self.draw()?,
                            _ => {},
                        }
                    },
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        self.running = false;
                        break;
                    },
                    Event::KeyUp { keycode: Some(keycode), keymod, .. } => {
                        let key = Keybinding {
                            key: keycode,
                            modifiers: keymod.into(),
                        };

                        // TODO im finding it instead of using hashmap get cause keymod hash is
                        // impossible to get right
                        if let Some((_, actions)) = self.keybindings.iter().find(|(x, _)| **x == key) {
                            for action in actions.0.clone() {
                                println!("Executing {action:?}");

                                action.execute(self)?;

                                // TODO is this delay enough?
                                std::thread::sleep(Duration::from_millis(5));
                            }
                        }

                        // // execute all actions in sequence
                        // if let Some(actions) = self.keybindings.get(&key).cloned() {
                        //     for action in actions.0 {
                        //         println!("Executing {action:?}");
                        //
                        //         action.execute(self)?;
                        //
                        //         // TODO is this delay enoguh?
                        //         std::thread::sleep(Duration::from_millis(5));
                        //     }
                        // }
                    },
                    Event::MouseMotion { x, y, .. } => {
                        self.mouse_position = Point::new(x.round_ties_even() as i32, y.round_ties_even() as i32);
                    },
                    _ => {}
                }
            }

            if self.update_requested {
                self.update_requested = false;
                self.draw()?;
            }

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        // nothing to draw when window is closed
        if self.window.is_none() {
            return Ok(());
        }

        let canvas = &mut self.window.as_mut().unwrap().canvas;

        // if i draw at edges i go offscreen
        let rect = self.rect.offset(2, 2);
        let center = rect.center();

        // make background transparent
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGBA(255, 0, 0, 128));

        // outline
        let outline = Rect::new(rect.x, rect.y, rect.w, rect.h);

        canvas.draw_rect(outline)?;
        canvas.draw_point(center)?;

        // TODO disable it based on area of a single square
        // do not draw the grid if too small of rectangle
        if rect.area() > 500 {
            // the 3x3 grid
            canvas.set_draw_color(Color::RGBA(0, 255, 0, 128));
            canvas.draw_line((rect.x + rect.w / 3, rect.y), (rect.x + rect.w / 3, rect.y + rect.h))?;
            canvas.draw_line((rect.x + rect.w / 3 * 2, rect.y), (rect.x + rect.w / 3 * 2, rect.y + rect.h))?;
            canvas.draw_line((rect.x, rect.y + rect.h / 3), (rect.x + rect.w, rect.y + rect.h / 3))?;
            canvas.draw_line((rect.x, rect.y + rect.h / 3 * 2), (rect.x + rect.w, rect.y + rect.h / 3 * 2))?;
        }

        canvas.present();

        Ok(())
    }

    pub fn draw_text_centered(&self, text: &str, x: i32, y: i32) -> Result<()> {
        todo!()
    }
}

// TODO font license notice somewhere
fn main() -> Result<()> {
    env_logger::init();

    let mut args = cli::Cli::parse();
    let cmd = args.cmd.take();

    match cmd {
        Some(cli::CliCommands::Configure) => {
            let _mouse = FakeMouse::new()?;

            println!("Fake mouse device has been created with name {:?}, now configure any options you want in your system settings.\n\nPress ENTER to close", "miceless");

            // wait for the user to press enter
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
        },
        Some(cli::CliCommands::Script(script)) => {
            // TODO should script mode load keybindings?
            let mut app = App::new(HashMap::new())?;

            let input = script.action_list
                .join(" ")
                .replace(",", ";"); // semicolon has to be escaped in shell so
            let action_list = input.parse::<ActionList>()
                .map_err(|x| anyhow!("{x}"))?;

            for action in action_list.0 {
                action.execute(&mut app)?;
            }
        },
        Some(cli::CliCommands::Actions) => {
            for (name, usage, help) in Action::get_actions() {
                if !help.is_empty() {
                    println!("/// {help}");
                }
                println!("{name} {usage}\n");
            }
        },
        None => {
            // TODO default keybindings and load from config file..
            let mut app = App::new(default_keybindings())?;
            app.open_window()?;
            app.main_loop()?;
        },
    }

    Ok(())
}

// NOTE this function is a separate function so it could be tested
fn default_keybindings() -> HashMap<Keybinding, ActionList> {
    HashMap::from([
        (Keybinding::from_str("1").unwrap(), "grid 3 3 0 0".parse::<ActionList>().unwrap()),
        (Keybinding::from_str("2").unwrap(), "grid 3 3 1 0".parse::<ActionList>().unwrap()),
        (Keybinding::from_str("3").unwrap(), "grid 3 3 2 0".parse::<ActionList>().unwrap()),
        (Keybinding::from_str("4").unwrap(), "grid 3 3 0 1".parse::<ActionList>().unwrap()),
        (Keybinding::from_str("5").unwrap(), "grid 3 3 1 1".parse::<ActionList>().unwrap()),
        (Keybinding::from_str("6").unwrap(), "grid 3 3 2 1".parse::<ActionList>().unwrap()),
        (Keybinding::from_str("7").unwrap(), "grid 3 3 0 2".parse::<ActionList>().unwrap()),
        (Keybinding::from_str("8").unwrap(), "grid 3 3 1 2".parse::<ActionList>().unwrap()),
        (Keybinding::from_str("9").unwrap(), "grid 3 3 2 2".parse::<ActionList>().unwrap()),
        //
        (Keybinding::from_str("SHIFT-RETURN").unwrap(), "hide; move_to_center; quit".parse::<ActionList>().unwrap()),
        (Keybinding::from_str("RETURN").unwrap(), "hide; move_to_center; click left; quit".parse::<ActionList>().unwrap()),
        (Keybinding::from_str("CTRL-RETURN").unwrap(), "hide; move_to_center; click right; quit".parse::<ActionList>().unwrap()),
        (Keybinding::from_str("ALT-RETURN").unwrap(), "hide; move_to_center; click middle; quit".parse::<ActionList>().unwrap()),
        (Keybinding::from_str("SHIFT-UP").unwrap(), "hide; scroll 0 100; show".parse::<ActionList>().unwrap()),
        (Keybinding::from_str("SHIFT-DOWN").unwrap(), "hide;scroll 0 -100; show".parse::<ActionList>().unwrap()),
    ])
}

#[test]
fn test_default_keybindings() {
    assert!(default_keybindings().len() > 0, "default keybindings are invalid");
}
