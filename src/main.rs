extern crate minifb;

use minifb::{Key, Window, WindowOptions, Scale, ScaleMode};

const WIDTH: usize = 160;
const HEIGHT: usize = 80;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let options = WindowOptions {
        borderless: true,
        title: true,
        resize: true,
        scale: Scale::X8,
        scale_mode: ScaleMode::Stretch,
        topmost: false,
        transparency: false,
        none: false
    };
    let mut window = Window::new(
        "GMTK Test",
        WIDTH,
        HEIGHT,
        options,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    enum Direction {
        Up,
        Down,
        Left,
        Right
    }
    let mut dir = Direction::Right;
    let mut position = 1;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::W) {
            dir = Direction::Up;
        } else if window.is_key_down(Key::S) {
            dir = Direction::Down;
        } else if window.is_key_down(Key::D) {
            dir = Direction::Right;
        } else if window.is_key_down(Key::A) {
            dir = Direction::Left;
        }
        buffer[position] = 0;
        match dir {
            Direction::Up => position -= WIDTH,
            Direction::Down => position += WIDTH,
            Direction::Left => position -= 1,
            Direction::Right => position += 1,
        }
        buffer[position] = 0xFFFFFF;
        window.
            update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
