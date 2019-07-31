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

    if args.len() < 3 {
        println!("Usage: cargo run /path/to/image.(png|jpg)");
    } else {
        run(
            Path::new(&args[1]),
            Path::new(&args[2]),
        )?;
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

fn run (playerImg: &Path, bulletImg: &Path) -> Result<(), String> {
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

    let mut player = Entity {
        x: 100,
        y: 100,
        dx: 4,
        dy: 4,
        health: 0,
        texture: texture_creator.load_texture(playerImg)?,
    };
    let mut bullet = Entity {
        x: 0,
        y: 0,
        dx: 16,
        dy: 0,
        health: 0,
        texture: texture_creator.load_texture(bulletImg)?,
    };
    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;
    let mut fire = 0;
    let mut event_pump = sdl_context.event_pump()?;
    canvas.set_draw_color(Color::RGB(98, 128, 255));

    'running: loop {
        canvas.clear();

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
                Event::KeyDown { keycode: Some(Keycode::LCtrl), .. } => {
                    fire = 1;
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
                Event::KeyUp { keycode: Some(Keycode::LCtrl), .. } => {
                    fire = 0;
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

        if fire == 1 && bullet.health == 0 {
            bullet.health = 1;
            bullet.x = player.x;
            bullet.y = player.y + 10;
        }
        if bullet.health == 1 {
            bullet.x = bullet.x + bullet.dx;
        }
        if bullet.x > 800 {
            bullet.health = 0;
        }

        let playerDest = Rect::new(
            player.x,
            player.y,
            player.texture.query().width,
            player.texture.query().height);
        canvas.copy(&player.texture, None, Some(playerDest))?;

        if bullet.health == 1 {
            let bulletDest = Rect::new(
                bullet.x,
                bullet.y,
                bullet.texture.query().width,
                bullet.texture.query().height);
            canvas.copy(&bullet.texture, None, Some(bulletDest))?;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

/* fn make_bullet(player: &Entity) -> Entity { */
    // Entity {
        // x: player.x,
        // y: player.y,
        // dx: 16,
        // dy: 0,
        // health: 1,

    // }
/* } */
