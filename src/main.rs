use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::env;
use std::path::Path;
use std::time::Duration;
use std::collections::HashMap;
use std::vec::Vec;

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

struct Entity<'a, 'b> {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    health: u8,
    reload: u8,
    texture: &'a Texture<'b>,
}

fn fire_bullet<'a, 'b>(texture: &'a Texture<'b>, player: &Entity) -> Entity<'a, 'b> {
    Entity {
        x: player.x,
        // y: player.y + ((player.texture.query().height - texture.query().height) / 2) as i32,
        y: player.y,
        dx: 16,
        dy: 0,
        health: 1,
        reload: 0,
        texture: texture,
    }
}

struct App<'a, 'b> {
    keyboard: HashMap<Keycode, bool>,
    player: Entity<'a, 'b>,
    bullets: Vec<Entity<'a, 'b>>,
}

fn init<'a, 'b>(texture: &'a Texture<'b>) -> App<'a, 'b> {
    App {
        keyboard: HashMap::new(),
        player: Entity {
            x: (texture.query().width / 2) as i32,
            y: (texture.query().width / 2) as i32,
            dx: 4,
            dy: 4,
            health: 0,
            reload: 0,
            texture: texture,
        },
        bullets: Vec::new(),
    }
}

fn render(canvas: &mut WindowCanvas, app: &App) -> Result<(), String> {
    let player_dest = Rect::new(
        app.player.x - (app.player.texture.query().width / 2) as i32,
        app.player.y - (app.player.texture.query().height / 2) as i32,
        app.player.texture.query().width,
        app.player.texture.query().height);
    canvas.copy(&app.player.texture, None, Some(player_dest))?;

    for bullet in &app.bullets {
        let bullet_dest = Rect::new(
            bullet.x - (bullet.texture.query().width / 2) as i32,
            bullet.y - (bullet.texture.query().height / 2) as i32,
            bullet.texture.query().width,
            bullet.texture.query().height);
        canvas.copy(&bullet.texture, None, Some(bullet_dest))?;
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
    let player_texture = texture_creator.load_texture(player_img)?;
    let bullet_texture = texture_creator.load_texture(bullet_img)?;

    let mut app = init(&player_texture);

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

        if app.player.reload > 0 {
            app.player.reload -= 1;
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
        if *app.keyboard.get(&Keycode::Space).unwrap_or(&false) && app.player.reload == 0 {
            app.player.reload = 8;
            app.bullets.push(fire_bullet(&bullet_texture, &app.player));
        }

        for bullet in &mut app.bullets {
            bullet.x += bullet.dx;
            bullet.y += bullet.dy;
        }
        app.bullets.retain(|bullet| bullet.x <= 800);
        render(&mut canvas, &app)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

