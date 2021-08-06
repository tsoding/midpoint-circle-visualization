extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

#[allow(dead_code)]
enum Mode {
    Points,
    Lines,
}

const MODE: Mode = Mode::Lines;
const CLEAR_FRAMES: bool = false;
const k: i32 = 10;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Circle", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;

    let cx = 200;
    let cy = 200;
    let r = 100;

    let mut x = r;
    let mut y = 0;
    let mut err = 0;

    let mut pause = false;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), ..  } => {
                    running = false;
                },

                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    x = r;
                    y = 0;
                    err = 0;
                },

                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    pause = !pause;
                }

                _ => ()
            }
        }

        if CLEAR_FRAMES {
            renderer.set_draw_color(Color::RGB(0, 0, 0));
            renderer.clear();
        }

        renderer.set_draw_color(Color::RGB(255, 0, 0));
        if x >= y {
            match MODE {
                Mode::Points => {
                    renderer.fill_rect(Rect::new(cx + x - k, cy + y - k, k as u32, k as u32));
                    renderer.fill_rect(Rect::new(cx + x - k, cy - y - k, k as u32, k as u32));

                    renderer.fill_rect(Rect::new(cx + y - k, cy + x - k, k as u32, k as u32));
                    renderer.fill_rect(Rect::new(cx + y - k, cy - x - k, k as u32, k as u32));

                    renderer.fill_rect(Rect::new(cx - y - k, cy + x - k, k as u32, k as u32));
                    renderer.fill_rect(Rect::new(cx - y - k, cy - x - k, k as u32, k as u32));

                    renderer.fill_rect(Rect::new(cx - x - k, cy + y - k, k as u32, k as u32));
                    renderer.fill_rect(Rect::new(cx - x - k, cy - y - k, k as u32, k as u32));
                },

                Mode::Lines => {
                    renderer.fill_rect(Rect::new(cx + x - k, cy - y - k, k as u32, 2 * y as u32));

                    renderer.fill_rect(Rect::new(cx + y - k, cy - x - k, k as u32, 2 * x as u32));

                    renderer.fill_rect(Rect::new(cx - y - k, cy - x - k, k as u32, 2 * x as u32));

                    renderer.fill_rect(Rect::new(cx - x - k, cy - y - k, k as u32, 2 * y as u32));
                },
            }

            if !pause {
                y += 1;
                err += 1 + 2 * y;
                if 2 * (err - x) + 1 > 0 {
                    x -= 1;
                    err += 1 - 2 * x;
                }
            }
        }

        renderer.present();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
