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
use rand::Rng;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run /path/to/image.(png|jpg)");
    } else {
        run(
            Path::new(&args[1]),
            Path::new(&args[2]),
            Path::new(&args[3]),
        )?;
    }

    Ok(())
}

enum Side {
    Player,
    Enemy,
}

struct Entity<'a> {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    health: u8,
    side: Side,
    reload: u8,
    texture: &'a Texture<'a>,
}

fn fire_bullet<'a>(texture: &'a Texture<'a>, x: i32, y: i32, side: Side) -> Entity<'a> {
    Entity {
        x,
        y,
        dx: 16,
        dy: 0,
        health: 1,
        side,
        reload: 0,
        texture: texture,
    }
}

fn spawn_enemy<'a>(texture: &'a Texture<'a>) -> Entity<'a> {
    Entity {
        x: rand::thread_rng().gen_range(700, 750),
        y: rand::thread_rng().gen_range(100, 500),
        dx: rand::thread_rng().gen_range(3, 5),
        dy: 0,
        health: 1,
        side: Side::Enemy,
        reload: 0,
        texture: texture,
    }
}

struct Stage<'a> {
    player: Entity<'a>,
    bullets: Vec<Entity<'a>>,
    enemies: Vec<Entity<'a>>,
    spawn: u8,
}

fn init_stage<'a>(texture: &'a Texture<'a>) -> Stage<'a> {
    Stage {
        player: Entity {
            x: (texture.query().width / 2) as i32,
            y: (texture.query().width / 2) as i32,
            dx: 4,
            dy: 4,
            health: 3,
            side: Side::Player,
            reload: 0,
            texture: texture,
        },
        bullets: Vec::new(),
        enemies: Vec::new(),
        spawn: 30,
    }
}

struct App {
    keyboard: HashMap<Keycode, bool>,
}

fn init_app() -> App {
    App {
        keyboard: HashMap::new(),
    }
}

fn render(canvas: &mut WindowCanvas, stage: &Stage) -> Result<(), String> {
    let player_dest = Rect::new(
        stage.player.x - (stage.player.texture.query().width / 2) as i32,
        stage.player.y - (stage.player.texture.query().height / 2) as i32,
        stage.player.texture.query().width,
        stage.player.texture.query().height);
    canvas.copy(&stage.player.texture, None, Some(player_dest))?;

    for bullet in &stage.bullets {
        let bullet_dest = Rect::new(
            bullet.x - (bullet.texture.query().width / 2) as i32,
            bullet.y - (bullet.texture.query().height / 2) as i32,
            bullet.texture.query().width,
            bullet.texture.query().height);
        canvas.copy(&bullet.texture, None, Some(bullet_dest))?;
    }

    for enemy in &stage.enemies {
        let enemy_dest = Rect::new(
            enemy.x - (enemy.texture.query().width / 2) as i32,
            enemy.y - (enemy.texture.query().height / 2) as i32,
            enemy.texture.query().width,
            enemy.texture.query().height);
        canvas.copy(&enemy.texture, None, Some(enemy_dest))?;
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

fn run (player_img: &Path, bullet_img: &Path, enemy_img: &Path) -> Result<(), String> {
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
    let enemy_texture = texture_creator.load_texture(enemy_img)?;

    let mut app = init_app();
    let mut stage = init_stage(&player_texture);

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
            stage.player.y = stage.player.y - stage.player.dy;
        }
        if *app.keyboard.get(&Keycode::Down).unwrap_or(&false) {
            stage.player.y = stage.player.y + stage.player.dy;
        }
        if *app.keyboard.get(&Keycode::Left).unwrap_or(&false) {
            stage.player.x = stage.player.x - stage.player.dx;
        }
        if *app.keyboard.get(&Keycode::Right).unwrap_or(&false) {
            stage.player.x = stage.player.x + stage.player.dx;
        }

        if stage.player.reload > 0 {
            stage.player.reload -= 1;
        }
        if *app.keyboard.get(&Keycode::Space).unwrap_or(&false) && stage.player.reload == 0 {
            stage.player.reload = 8;
            stage.bullets.push(fire_bullet(&bullet_texture, stage.player.x, stage.player.y, Side::Player));
        }

        if stage.spawn > 0 {
            stage.spawn -= 1;
        }
        if stage.spawn == 0 && stage.enemies.len() < 5 {
            stage.spawn = 30 + rand::thread_rng().gen_range(0, 30);
            stage.enemies.push(spawn_enemy(&enemy_texture));
        }

        for enemy in &mut stage.enemies {
            enemy.x -= enemy.dx;
            enemy.y -= enemy.dy;
        }
        stage.enemies.retain(|enemy| enemy.x > -80);

        for bullet in &mut stage.bullets {
            bullet.x += bullet.dx;
            bullet.y += bullet.dy;
        }
        stage.bullets.retain(|bullet| bullet.x <= 800);

        render(&mut canvas, &stage)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

