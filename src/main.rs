mod util;
mod mouse;
mod cli;
mod assets;

use util::*;
use anyhow::{anyhow, Result};
use sdl3::{event::Event, keyboard::{Keycode, Mod}, pixels::Color, render::{TextureQuery, WindowCanvas}, video::WindowFlags};
use std::time::Duration;
use crate::mouse::{FakeMouse, MouseKey};

struct App<'a> {
    sdl_context: sdl3::Sdl,
    ttf_context: sdl3::ttf::Sdl3TtfContext,
    video_subsystem: sdl3::VideoSubsystem,

    canvas: WindowCanvas,
    font: sdl3::ttf::Font<'a>,

    // TODO
    // keybindings: Vec<>,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self> {
        todo!()
    }

    pub fn draw_text_centered(&self, text: &str, x: i32, y: i32) -> Result<()> {
        todo!()
    }
}

// TODO font license notice somewhere
// TODO provide a mode where only fake mouse is created to disable acceleration...
// TODO wrap the window in a struct
// TODO make enum for commands that can be executed like ClickAt, ScrollAt, Drag, etc
fn main() -> Result<()> {
    env_logger::init();

    // NOTE it cannot be used right after creation so create it in advance
    let mut fake_mouse = FakeMouse::new()?;

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl3::ttf::init()?;

    let window = video_subsystem.window("Miceless", 800, 600)
        .set_flags(WindowFlags::FULLSCREEN | WindowFlags::BORDERLESS | WindowFlags::ALWAYS_ON_TOP | WindowFlags::UTILITY | WindowFlags::TRANSPARENT)
        .build()
        .unwrap();

    // TODO display always gets the same monitor as get_primary_display
    let display = window.get_display()?;
    // TODO is this scaled if scaling is enabled?
    let display_size = display.get_bounds()?;

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    // TODO font size
    // SAFETY: it cannot fail as FONT is static
    let font = ttf_context.load_font_from_iostream(sdl3::iostream::IOStream::from_bytes(assets::FONT).unwrap(), 24.0)?;

    // let surface = font
    //     .render("Hello Rust!")
    //     .blended(Color::RGBA(255, 0, 0, 255))?;
    // let texture = texture_creator
    //     .create_texture_from_surface(&surface)?;

    let mut mouse_key: Option<MouseKey> = None;
    let mut mouse_current: Point = Point::default();

    let mut rect_old: Option<Rect> = None;
    let mut rect: Rect = Rect::new(0, 0, display_size.w, display_size.h); 

    // // TODO draw text at specified location
    // let TextureQuery { width, height, .. } = texture.query();
    //
    // canvas.copy(&texture, None, Some(Rect::new(100, 100, width as i32, height as i32).into()))?;
    // canvas.present();

    draw(&mut canvas, &rect)?;

    // TODO these should be configurable
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyUp { keycode: Some(x), keymod, .. } => {
                    let redraw = match x {
                        Keycode::Up => {
                            rect_old = Some(rect);
                            rect = rect.half(Direction4::Top);

                            true
                        },
                        Keycode::Down => {
                            rect_old = Some(rect);
                            rect = rect.half(Direction4::Bottom);

                            true
                        },
                        Keycode::Left => {
                            rect_old = Some(rect);
                            rect = rect.half(Direction4::Left);

                            true
                        },
                        Keycode::Right => {
                            rect_old = Some(rect);
                            rect = rect.half(Direction4::Right);

                            true
                        },
                        Keycode::_1 => {
                            rect_old = Some(rect);
                            rect = rect.third(Direction8::TopLeft);

                            true
                        },
                        Keycode::_2 => {
                            rect_old = Some(rect);
                            rect = rect.third(Direction8::Top);

                            true
                        },
                        Keycode::_3 => {
                            rect_old = Some(rect);
                            rect = rect.third(Direction8::TopRight);

                            true
                        },
                        Keycode::_4 => {
                            rect_old = Some(rect);
                            rect = rect.third(Direction8::Left);

                            true
                        },
                        Keycode::_5 => {
                            rect_old = Some(rect);
                            rect = rect.third(Direction8::Center);

                            true
                        },
                        Keycode::_6 => {
                            rect_old = Some(rect);
                            rect = rect.third(Direction8::Right);

                            true
                        },
                        Keycode::_7 => {
                            rect_old = Some(rect);
                            rect = rect.third(Direction8::BottomLeft);

                            true
                        },
                        Keycode::_8 => {
                            rect_old = Some(rect);
                            rect = rect.third(Direction8::Bottom);

                            true
                        },
                        Keycode::_9 => {
                            rect_old = Some(rect);
                            rect = rect.third(Direction8::BottomRight);

                            true
                        },

                        // basically undo
                        Keycode::U => {
                            // allow undo only once
                            if let Some(rect_old) = rect_old {
                                rect = rect_old;
                            }

                            true
                        },

                        Keycode::Return | Keycode::KpEnter => {
                            // TODO more testing as i dont want these to clash
                            if keymod.contains(Mod::LSHIFTMOD) || keymod.contains(Mod::RSHIFTMOD) {
                                mouse_key = Some(MouseKey::Right);
                            } else {
                                mouse_key = Some(MouseKey::Left);
                            }

                            break 'running
                        }

                        _ => false,
                    };

                    if redraw {
                        draw(&mut canvas, &rect)?;
                    }
                },
                Event::MouseMotion { x, y, .. } => {
                    mouse_current = Point::new(x.round_ties_even() as i32, y.round_ties_even() as i32);
                },
                Event::MouseButtonUp { .. } | Event::MouseButtonDown { .. } => {
                    break 'running
                }
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    // close window before moving the mouse
    drop(canvas);

    std::thread::sleep(Duration::from_millis(10));

    if let Some(mouse_key) = mouse_key {
        let center = rect.center();

        // move the mouse to the target
        fake_mouse.rel_move(center.x - mouse_current.x, center.y - mouse_current.y)?;
        // fake_mouse.reset_position()?;
        // fake_mouse.sleep();
        // std::thread::sleep(std::time::Duration::from_millis(100));
        // fake_mouse.rel_move(center.x, center.y)?;

        // delay so it works properly
        // fake_mouse.sleep();

        // fake_mouse.press(evdev::KeyCode::BTN_RIGHT)?;
        //
        // fake_mouse.release(evdev::KeyCode::BTN_RIGHT)?;

        // fake_mouse.click(mouse_key, None)?;
    }

    Ok(())
}

fn draw(canvas: &mut WindowCanvas, rect: &Rect) -> Result<()> {
    // if i draw at edges i go offscreen
    let rect = rect.offset(2, 2);
    let center = rect.center();

    // make background transparent
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGBA(255, 0, 0, 128));

    // outline
    let outline = Rect::new(rect.x, rect.y, rect.w, rect.h);

    canvas.draw_rect(outline)?;
    canvas.draw_point(center)?;

    canvas.draw_debug_text("hello", center)?;

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
