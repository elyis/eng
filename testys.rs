use rand::{thread_rng, Rng};
use std::{thread, time};
use sfml::{
    graphics::{
        Color, Font, PrimitiveType, Rect, RenderStates, RenderTarget, RenderWindow, Text, Texture,
        Transform, Vertex, View, Sprite,
    },
    system::{Clock, Vector2, Vector2f, Vector2i},
    window::{mouse::Button, ContextSettings, Event, Key, Style, VideoMode},
};

include!("../example_common.rs");

mod Animation;

fn fconv(in_: Vector2i) -> Vector2f {
    Vector2f {
        x: in_.x as f32,
        y: in_.y as f32,
    }
}

fn main() {
    let native_mode = VideoMode::desktop_mode();
    let mut window = RenderWindow::new(
        native_mode,
        "Spritemark",
        Style::NONE,
        &ContextSettings::default(),
    );
    window.set_position(Vector2::new(0, 0));
    window.set_vertical_sync_enabled(true);

    let texture = Texture::from_file(example_res!("anim_man.png")).unwrap();
    let back_texture = Texture::from_file(example_res!("background.jpeg")).unwrap();

    let mut back_sprite = Sprite::new();
    back_sprite.set_texture(&back_texture, true);
    let mut animation = Animation::Animation::new(&texture, 5, 40, 81, 190);
    let mut mp = window.mouse_position();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => window.close(),
                Event::Resized { width, height } => {
                    window.set_view(&View::from_rect(&Rect::new(
                        0.,
                        0.,
                        width as f32,
                        height as f32,
                    )));
                }
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        window.draw(&back_sprite);
        animation.draw(&mut window,mp.x,mp.y);
        mp = window.mouse_position();
        window.display();
    }
}