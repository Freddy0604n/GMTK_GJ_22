extern crate minifb;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

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
        none: false,
    };
    let mut window = Window::new("GMTK Test", WIDTH, HEIGHT, options).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600*2)));
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    let mut dir = Direction::Right;
    let mut position = 1;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::W) { // Controls
            dir = Direction::Up;
        } else if window.is_key_down(Key::S) {
            dir = Direction::Down;
        } else if window.is_key_down(Key::D) {
            dir = Direction::Right;
        } else if window.is_key_down(Key::A) {
            dir = Direction::Left;
        }

        buffer[position] = 0; // clear the previous position
        match dir {
            Direction::Up => {
                if (position as i32 - WIDTH as i32) < 0 { // if the position needs to roll over
                    position = WIDTH * HEIGHT - (WIDTH - position);
                } else {
                    position -= WIDTH;
                }
            }
            Direction::Down => {
                if position + WIDTH > WIDTH * HEIGHT {  // ''
                    position = (position + WIDTH) - WIDTH * HEIGHT;
                } else {
                    position += WIDTH;
                }
            }
            Direction::Left => {if position % WIDTH == 0 && position != 0 && position != WIDTH { // ''
                position = position - 1 + WIDTH;
            } else if position == 0{
                position = WIDTH - 1;
            } else {
                position -= 1;
            }},
            Direction::Right => {if (position + 1) % WIDTH == 0 { // ''
                position = position + 1 - WIDTH;
            } else {
                position += 1;
            }},
        }
        buffer[position] = 0xFFFF00;
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
