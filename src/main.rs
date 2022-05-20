extern crate minifb;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 320;
const HEIGHT: usize = 160;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let map_width = WIDTH * 2;
    let map_height = HEIGHT + 20;
    let mut map: Vec<u32> = vec![0; map_width * map_height];
    for i in 0..map_height * map_width {
        if i % 6 == 0 {
            map[i] = 0xFFFF00;
        }
    }

    let mut camera = 0;

    let options = WindowOptions {
        borderless: true,
        title: true,
        resize: true,
        scale: Scale::X4,
        scale_mode: ScaleMode::AspectRatioStretch,
        topmost: false,
        transparency: false,
        none: false,
    };

    let mut window = Window::new("background test", WIDTH, HEIGHT, options).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                buffer[i * WIDTH + j] = map[camera + j + i * map_width];
            }
        }
        if window.is_key_down(Key::Up) {
            if camera as i32 - map_width as i32 >= 0 {
                camera -= map_width;
            }
        } 
        if window.is_key_down(Key::Down) {
            if (camera + HEIGHT * map_width) < (map_height * map_width + 1) {
                camera += map_width;
            }
        } 
        if window.is_key_down(Key::Right) {
            camera += 1;
        } 
        if window.is_key_down(Key::Left) {
            camera -= 1;
        } 
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
