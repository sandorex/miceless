mod util;
mod mouse;
mod cli;
mod assets;
mod action;

use action::Action;
use util::*;
use anyhow::{anyhow, Result};
use sdl3::{event::Event, keyboard::{Keycode, Mod}, pixels::Color, render::{TextureQuery, WindowCanvas}, video::WindowFlags};
use std::{collections::HashMap, hash::Hash, rc::Rc, time::Duration};
use crate::mouse::{FakeMouse, MouseKey};

/// Combines SDL3 `Keycode` and `Mod` into one hashable struct
#[derive(Debug, Clone, Copy)]
pub struct Keybinding {
    pub key: Keycode,
    pub modifiers: Mod,
}

impl PartialEq for Keybinding {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.modifiers == other.modifiers
    }
}

impl Eq for Keybinding {}

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
    pub keybindings: HashMap<Keybinding, Vec<Rc<dyn Action>>>,
}

impl<'a> App<'a> {
    pub fn new(keybindings: HashMap<Keybinding, Vec<Rc<dyn Action>>>) -> Result<Self> {
        let sdl_context = sdl3::init().unwrap();
        let ttf_context = sdl3::ttf::init()?;
        let video_subsystem = sdl_context.video().unwrap();

        // TODO font size
        // SAFETY: it cannot fail as FONT is static
        let font = ttf_context.load_font_from_iostream(sdl3::iostream::IOStream::from_bytes(assets::FONT).unwrap(), 24.0)?;

        let mouse = FakeMouse::new()?;

        let window = Window::new(&video_subsystem)?;

        // TODO i do not know if this gets proper size of the monitor as both my monitors are same
        // resolution
        let output_size = window.canvas.output_size()?;
        dbg!(&output_size);

        let full_rect = Rect::new(0, 0, output_size.0.try_into().unwrap(), output_size.1.try_into().unwrap()); // TODO remove unwraps

        let app = Self {
            sdl_context,
            ttf_context,
            video_subsystem,
            font,
            mouse,
            full_rect,
            rect: full_rect,
            window: Some(window),
            mouse_position: Default::default(),
            running: true,
            keybindings,
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
                            modifiers: keymod,
                        };

                        // execute all actions in sequence
                        if let Some(actions) = self.keybindings.get(&key).cloned() {
                            for action in actions {
                                action.execute(self)?;

                                // TODO is this delay enoguh?
                                std::thread::sleep(Duration::from_millis(5));
                            }
                        }
                    },
                    Event::MouseMotion { x, y, .. } => {
                        self.mouse_position = Point::new(x.round_ties_even() as i32, y.round_ties_even() as i32);
                    },
                    // TODO if i enable this then clicking via actions would close the window and
                    // confuse the user
                    // Event::MouseButtonUp { .. } | Event::MouseButtonDown { .. } => {
                    //     self.running = false;
                    //     break
                    // }
                    _ => {}
                }
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
// TODO provide a mode where only fake mouse is created to disable acceleration...
fn main() -> Result<()> {
    env_logger::init();

    // TODO parse keybinding from string (ex. SHIFT- ALT- K)
    let map = HashMap::from([
        (Keybinding { key: Keycode::A, modifiers: Mod::NOMOD }, action::parse_action_list("show false; click left; quit")?),
    ]);

    let mut app = App::new(map)?;
    app.main_loop()?;

    Ok(())
}
