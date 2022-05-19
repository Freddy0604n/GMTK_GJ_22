extern crate minifb;

use minifb::{Key, Window, WindowOptions, Scale, ScaleMode};

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let options = WindowOptions {
        borderless: true,
        title: true,
        resize: true,
        scale: Scale::X1,
        scale_mode: ScaleMode::Center,
        topmost: true,
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

    let mut counter = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0xFF0000;
        }

        window.
            update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
