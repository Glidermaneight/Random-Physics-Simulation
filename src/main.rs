use crate::Forces::Forces::Gravity;
extern crate sdl2;
pub mod Circle;
pub mod Forces;
pub mod Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut circle = Circle::Circle::Circle::new(50, 50, 50, 0);
    let mut rect = Rect::Rect::Rect::new(sdl2::rect::Rect::new(400, 40, 100, 100), 0);
    let floor = Rect::Rect::Rect::new(sdl2::rect::Rect::new(0, 500, 800, 100), 0);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        match circle.detect_collision(&floor) {
            true => {}
            false => circle.fall(),
        }
        match rect.check_for_collision(&floor) || rect.aabb(floor.get_rect()) {
            true => {}
            false => rect.fall(),
        }
        canvas.set_draw_color(Color::RGB(255, 255, 0));
        let _ = canvas.fill_rect(rect.get_rect());
        let _ = Circle::fill_circle(&mut canvas, &mut circle.clone());
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let _ = canvas.fill_rect(floor.get_rect());
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
