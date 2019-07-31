use sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::rect::Rect;
use sdl2::render::Texture;
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

struct Entity<'a> {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    health: u8,
    texture: Texture<'a>,
}

fn run (png: &Path) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    ::sdl2::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "linear");

    let window = video_subsystem
        .window("2d shoot", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut player = Entity {
        x: 100,
        y: 100,
        dx: 4,
        dy: 4,
        health: 0,
        texture: texture_creator.load_texture(png)?,
    };
    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;
    let mut event_pump = sdl_context.event_pump()?;
    // let mut i = 0;
    'running: loop {
        /* i = (i + 1) % 255; */
        /* canvas.set_draw_color(Color::RGB(i, 64, 255 - i)); */
        canvas.set_draw_color(Color::RGB(98, 128, 255));
        canvas.clear();

        let dest = Rect::new(
            player.x,
            player.y,
            player.texture.query().width,
            player.texture.query().height);
        canvas.copy(&player.texture, None, Some(dest))?;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    up = 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    down = 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    left = 1;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    right = 1;
                },
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    up = 0;
                },
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                    down = 0;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    left = 0;
                },
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    right = 0;
                },
                _ => {}
            }
        }

        if up == 1 {
            player.y = player.y - player.dy;
        }
        if down == 1 {
            player.y = player.y + player.dy;
        }
        if left == 1 {
            player.x = player.x - player.dx;
        }
        if right == 1 {
            player.x = player.x + player.dx;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
