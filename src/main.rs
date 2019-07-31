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

struct App {
    up: u8,
    down: u8,
    left: u8,
    right: u8,
    fire: u8,
}

fn init() -> App {
    App {
        up: 0,
        down: 0,
        left: 0,
        right: 0,
        fire: 0,
    }
}

fn view(app: &App) {
}

enum Msg {
    KeyDown(Keycode),
    KeyUp(Keycode),
}

fn update(app: &mut App, msg: &Msg) {
    match msg {
        Msg::KeyDown (Keycode::Up) => {
            app.up = 1;
        },
        Msg::KeyDown (Keycode::Down) => {
            app.down = 1;
        },
        Msg::KeyDown (Keycode::Left) => {
            app.left = 1;
        },
        Msg::KeyDown (Keycode::Right) => {
            app.right = 1;
        },
        Msg::KeyDown (Keycode::Space) => {
            app.fire = 1;
        },
        Msg::KeyUp (Keycode::Up) => {
            app.up = 0;
        },
        Msg::KeyUp (Keycode::Down) => {
            app.down = 0;
        },
        Msg::KeyUp (Keycode::Left) => {
            app.left = 0;
        },
        Msg::KeyUp (Keycode::Right) => {
            app.right = 0;
        },
        Msg::KeyUp (Keycode::Space) => {
            app.fire = 0;
        },
        _ => {}
    }
}

fn run (player_img: &Path, bullet_img: &Path) -> Result<(), String> {
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

    let mut app = init();

    let mut player = Entity {
        x: 100,
        y: 100,
        dx: 4,
        dy: 4,
        health: 0,
        texture: texture_creator.load_texture(player_img)?,
    };
    let mut bullet = Entity {
        x: 0,
        y: 0,
        dx: 16,
        dy: 0,
        health: 0,
        texture: texture_creator.load_texture(bullet_img)?,
    };
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
                Event::KeyDown { keycode: Some(code), .. } => {
                    update(&mut app, &Msg::KeyDown(code));
                },
                Event::KeyUp { keycode: Some(code), .. } => {
                    update(&mut app, &Msg::KeyUp(code));
                },
                _ => {}
            }
        }

        if app.up == 1 {
            player.y = player.y - player.dy;
        }
        if app.down == 1 {
            player.y = player.y + player.dy;
        }
        if app.left == 1 {
            player.x = player.x - player.dx;
        }
        if app.right == 1 {
            player.x = player.x + player.dx;
        }

        if app.fire == 1 && bullet.health == 0 {
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

        let player_dest = Rect::new(
            player.x,
            player.y,
            player.texture.query().width,
            player.texture.query().height);
        canvas.copy(&player.texture, None, Some(player_dest))?;

        if bullet.health == 1 {
            let bullet_dest = Rect::new(
                bullet.x,
                bullet.y,
                bullet.texture.query().width,
                bullet.texture.query().height);
            canvas.copy(&bullet.texture, None, Some(bullet_dest))?;
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
