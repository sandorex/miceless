mod util;
mod mouse;
mod cli;
mod assets;
mod actions;

use actions::{Action, Click};
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

pub struct App<'a> {
    pub sdl_context: sdl3::Sdl,
    pub ttf_context: sdl3::ttf::Sdl3TtfContext,
    pub video_subsystem: sdl3::VideoSubsystem,
    pub canvas: WindowCanvas,
    pub font: sdl3::ttf::Font<'a>,

    pub old_rect: Option<Rect>,
    pub rect: Rect,
    pub mouse: FakeMouse,
    pub mouse_position: Point,
    pub running: bool,
    pub keybindings: HashMap<Keybinding, Vec<Rc<dyn Action>>>,
}

impl<'a> App<'a> {
    pub fn new(keybindings: HashMap<Keybinding, Vec<Rc<dyn Action>>>) -> Result<Self> {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context = sdl3::ttf::init()?;

        let window = video_subsystem.window("Miceless", 800, 600)
            .set_flags(WindowFlags::FULLSCREEN | WindowFlags::NOT_FOCUSABLE | WindowFlags::BORDERLESS | WindowFlags::ALWAYS_ON_TOP | WindowFlags::UTILITY | WindowFlags::TRANSPARENT)
            .build()
            .unwrap();

        // TODO display is always the same as get_primary_display
        let display = window.get_display()?;
        let rect = Rect::from(display.get_bounds()?);

        let mut canvas = window.into_canvas();
        let texture_creator = canvas.texture_creator();

        // TODO font size
        // SAFETY: it cannot fail as FONT is static
        let font = ttf_context.load_font_from_iostream(sdl3::iostream::IOStream::from_bytes(assets::FONT).unwrap(), 24.0)?;

        let mouse = FakeMouse::new()?;

        Ok(Self {
            sdl_context,
            ttf_context,
            video_subsystem,
            canvas,
            font,
            mouse,
            rect,
            old_rect: None,
            mouse_position: Default::default(),
            running: true,
            keybindings,
        })
    }

    pub fn main_loop(&mut self) -> Result<()> {
        // // draw initial grid
        // self.draw()?;

        let mut event_pump = self.sdl_context.event_pump().unwrap();
        while self.running {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Window { timestamp, window_id, win_event } => {
                        match win_event {
                            sdl3::event::WindowEvent::Hidden => println!("hidden"),
                            sdl3::event::WindowEvent::Shown => {
                                self.draw()?;
                                println!("shown")
                            },
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
                                std::thread::sleep(Duration::from_millis(500));
                            }
                        }
                    },
                    Event::MouseMotion { x, y, .. } => {
                        self.mouse_position = Point::new(x.round_ties_even() as i32, y.round_ties_even() as i32);
                    },
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
        // if i draw at edges i go offscreen
        let rect = self.rect.offset(2, 2);
        let center = rect.center();

        // make background transparent
        self.canvas.set_draw_color(Color::RGBA(0, 128, 0, 128));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGBA(255, 0, 0, 128));

        // outline
        let outline = Rect::new(rect.x, rect.y, rect.w, rect.h);

        self.canvas.draw_rect(outline)?;
        self.canvas.draw_point(center)?;

        // do not draw the grid if too small of rectangle
        if rect.area() > 500 {
            // the 3x3 grid
            self.canvas.set_draw_color(Color::RGBA(0, 255, 0, 128));
            self.canvas.draw_line((rect.x + rect.w / 3, rect.y), (rect.x + rect.w / 3, rect.y + rect.h))?;
            self.canvas.draw_line((rect.x + rect.w / 3 * 2, rect.y), (rect.x + rect.w / 3 * 2, rect.y + rect.h))?;
            self.canvas.draw_line((rect.x, rect.y + rect.h / 3), (rect.x + rect.w, rect.y + rect.h / 3))?;
            self.canvas.draw_line((rect.x, rect.y + rect.h / 3 * 2), (rect.x + rect.w, rect.y + rect.h / 3 * 2))?;
        }

        self.canvas.present();

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

    use crate::actions::{Click, Window};

    let map = HashMap::from([
        (Keybinding { key: Keycode::A, modifiers: Mod::RSHIFTMOD }, vec![
            Rc::new(Window(false)) as Rc<dyn Action>,
            // Rc::new(Click(MouseKey::Left, None)) as Rc<dyn Action>,
            Rc::new(Window(true)) as Rc<dyn Action>,
        ]),
    ]);

    let mut app = App::new(map)?;
    app.main_loop()?;

    Ok(())
}
