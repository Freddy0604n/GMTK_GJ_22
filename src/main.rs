extern crate minifb;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut background: Vec<u32> = vec![0; WIDTH * HEIGHT * 2];
    let options = WindowOptions {
        borderless: true,
        title: true,
        resize: true,
        scale: Scale::X2,
        scale_mode: ScaleMode::Stretch,
        topmost: false,
        transparency: false,
        none: false,
    };

    let mut window = Window::new("background test", WIDTH, HEIGHT, options).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(166000)));

    for i in 0 .. WIDTH * HEIGHT * 2 {
        if i % 5 != 0 {
            background[i] = 0xFFFFFF;
        }
        if i as isize -1 % WIDTH as isize == 0 {
            background[i] = 0xFF0000;
        }
    }
    let position = 0;
    while window.is_open() {
        let mut line = 0;
        for i in 0 .. WIDTH * HEIGHT {
            if i % WIDTH == 0 {
                line += 1;
            }
            buffer[i] = background[i];
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
