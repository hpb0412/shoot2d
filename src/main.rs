use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::env;
use std::path::Path;
use std::time::Duration;
use std::collections::HashMap;

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

struct App<'a> {
    keyboard: HashMap<Keycode, bool>,
    player: Entity<'a>,
    bullet: Entity<'a>,
}

fn init<'a>(texture_creator: &'a TextureCreator<WindowContext>, player_img: &Path, bullet_img: &Path) -> Result<App<'a>, String> {
    Ok(App {
        keyboard: HashMap::new(),
        player: Entity {
            x: 100,
            y: 100,
            dx: 4,
            dy: 4,
            health: 0,
            texture: texture_creator.load_texture(player_img)?,
        },
        bullet: Entity {
            x: 0,
            y: 0,
            dx: 16,
            dy: 0,
            health: 0,
            texture: texture_creator.load_texture(bullet_img)?,
        },
    })
}

fn render(canvas: &mut WindowCanvas, app: &App) -> Result<(), String> {
    let player_dest = Rect::new(
        app.player.x,
        app.player.y,
        app.player.texture.query().width,
        app.player.texture.query().height);
    canvas.copy(&app.player.texture, None, Some(player_dest))?;

    if app.bullet.health == 1 {
        let bullet_dest = Rect::new(
            app.bullet.x,
            app.bullet.y,
            app.bullet.texture.query().width,
            app.bullet.texture.query().height);
        canvas.copy(&app.bullet.texture, None, Some(bullet_dest))?;
    }

    canvas.present();
    Ok(())
}

enum Msg {
    KeyDown(Keycode),
    KeyUp(Keycode),
}

fn update(app: &mut App, msg: Msg) {
    match msg {
        Msg::KeyDown(code) => {
            app.keyboard.insert(code, true);
        },
        Msg::KeyUp(code) => {
            app.keyboard.insert(code, false);
        },
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

    let mut app = init(&texture_creator, player_img, bullet_img)?;

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
                    update(&mut app, Msg::KeyDown(code));
                },
                Event::KeyUp { keycode: Some(code), .. } => {
                    update(&mut app, Msg::KeyUp(code));
                },
                _ => {}
            }
        }

        if *app.keyboard.get(&Keycode::Up).unwrap_or(&false) {
            app.player.y = app.player.y - app.player.dy;
        }
        if *app.keyboard.get(&Keycode::Down).unwrap_or(&false) {
            app.player.y = app.player.y + app.player.dy;
        }
        if *app.keyboard.get(&Keycode::Left).unwrap_or(&false) {
            app.player.x = app.player.x - app.player.dx;
        }
        if *app.keyboard.get(&Keycode::Right).unwrap_or(&false) {
            app.player.x = app.player.x + app.player.dx;
        }
        if *app.keyboard.get(&Keycode::Space).unwrap_or(&false) && app.bullet.health == 0 {
            app.bullet.health = 1;
            app.bullet.x = app.player.x;
            app.bullet.y = app.player.y + 10;
        }
        if app.bullet.health == 1 {
            app.bullet.x = app.bullet.x + app.bullet.dx;
        }
        if app.bullet.x > 800 {
            app.bullet.health = 0;
        }

        render(&mut canvas, &app)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

