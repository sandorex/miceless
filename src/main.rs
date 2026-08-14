mod util;

use util::*;
use anyhow::{anyhow, Result};
use sdl3::{event::Event, keyboard::Keycode, pixels::Color, render::WindowCanvas, video::WindowFlags};
use std::time::Duration;

fn main() -> Result<()> {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Miceless", 800, 600)
        .set_flags(WindowFlags::FULLSCREEN | WindowFlags::BORDERLESS | WindowFlags::ALWAYS_ON_TOP | WindowFlags::UTILITY | WindowFlags::TRANSPARENT)
        .build()
        .unwrap();

    // TODO display always gets the same monitor as get_primary_display
    let display = window.get_display()?;
    // TODO is this scaled if scaling is enabled?
    let display_size = display.get_bounds()?;
    println!("display: {:?} ({:?})", display.get_name(), display_size);

    let mut canvas = window.into_canvas();

    let mut mouse_x: i32 = 0;
    let mut mouse_y: i32 = 0;

    let mut rect: Rect = Rect::new(0, 0, display_size.w, display_size.h); 

    draw(&mut canvas, &rect)?;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyUp { keycode: Some(x), .. } => {
                    match x {
                        Keycode::Up => {
                            rect = rect.half(Direction4::Top);

                            // redraw
                            draw(&mut canvas, &rect)?;
                        },
                        Keycode::Down => {
                            rect = rect.half(Direction4::Bottom);

                            // redraw
                            draw(&mut canvas, &rect)?;
                        },
                        Keycode::Left => {
                            rect = rect.half(Direction4::Left);

                            // redraw
                            draw(&mut canvas, &rect)?;
                        },
                        Keycode::Right => {
                            rect = rect.half(Direction4::Right);

                            // redraw
                            draw(&mut canvas, &rect)?;
                        },
                        Keycode::_1 => {
                            rect = rect.third(Direction8::TopLeft);

                            // redraw
                            draw(&mut canvas, &rect)?;
                        },
                        Keycode::_2 => {
                            rect = rect.third(Direction8::Top);

                            // redraw
                            draw(&mut canvas, &rect)?;
                        },
                        Keycode::_3 => {
                            rect = rect.third(Direction8::TopRight);

                            // redraw
                            draw(&mut canvas, &rect)?;
                        },
                        Keycode::_4 => {
                            rect = rect.third(Direction8::Left);

                            // redraw
                            draw(&mut canvas, &rect)?;
                        },
                        Keycode::_5 => {
                            rect = rect.third(Direction8::Center);

                            // redraw
                            draw(&mut canvas, &rect)?;
                        },
                        Keycode::_6 => {
                            rect = rect.third(Direction8::Right);

                            // redraw
                            draw(&mut canvas, &rect)?;
                        },
                        Keycode::_7 => {
                            rect = rect.third(Direction8::BottomLeft);

                            // redraw
                            draw(&mut canvas, &rect)?;
                        },
                        Keycode::_8 => {
                            rect = rect.third(Direction8::Bottom);

                            // redraw
                            draw(&mut canvas, &rect)?;
                        },
                        Keycode::_9 => {
                            rect = rect.third(Direction8::BottomRight);

                            // redraw
                            draw(&mut canvas, &rect)?;
                        },
                        _ => {},
                    }
                },
                Event::MouseMotion { x, y, .. } => {
                    mouse_x = x.floor() as i32;
                    mouse_y = y.floor() as i32;
                },
                Event::MouseButtonUp { .. } | Event::MouseButtonDown { .. } => {
                    break 'running
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    // close window before moving the mouse
    drop(canvas);

    println!("mouse at {mouse_x}x{mouse_y}");

    // std::thread::sleep(std::time::Duration::from_millis(1500));


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
    // let crosshair = Rect::new_centered(center.x, center.y, 30, 30);

    // canvas.draw_rect(crosshair)?;

    // if outline.area() > crosshair.area() + (40 * 40) {
    canvas.draw_rect(outline)?;
    canvas.draw_point(center)?;

    // canvas.draw_line((center.x, rect.y), (center.x, crosshair.y))?; // top
    // canvas.draw_line((center.x, center.y + crosshair.h / 2), (center.x, rect.y + rect.h))?; // bottom
    // canvas.draw_line((rect.x, center.y), (crosshair.x, center.y))?; // left
    // canvas.draw_line((crosshair.x + crosshair.w, center.y), (rect.x + rect.w, center.y))?; // right

    // do not draw the grid if too small of rectangle
    if rect.area() > 500 {
        // the 3x3 grid
        canvas.set_draw_color(Color::RGBA(0, 255, 0, 128));
        canvas.draw_line((rect.x + rect.w / 3, rect.y), (rect.x + rect.w / 3, rect.y + rect.h))?;
        canvas.draw_line((rect.x + rect.w / 3 * 2, rect.y), (rect.x + rect.w / 3 * 2, rect.y + rect.h))?;
        canvas.draw_line((rect.x, rect.y + rect.h / 3), (rect.x + rect.w, rect.y + rect.h / 3))?;
        canvas.draw_line((rect.x, rect.y + rect.h / 3 * 2), (rect.x + rect.w, rect.y + rect.h / 3 * 2))?;
    }
    // }

    canvas.present();

    Ok(())
}
