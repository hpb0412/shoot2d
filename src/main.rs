use sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::rect::Rect;
use std::env;
use std::path::Path;
use std::time::Duration;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/image.(png|jpg)");
    } else {
        run(Path::new(&args[1]))?;
    }

    Ok(())
}

fn run (png: &Path) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("2d shoot", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(png)?;

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let dest = Rect::new(0, 0, 400, 300);
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        canvas.copy(&texture, None, Some(dest))?;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
