extern crate minifb;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 320;
const HEIGHT: usize = 160;

fn main() {
    let mut camera: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let map_width = WIDTH * 2;
    let map_height = HEIGHT + 20;
    let mut map: Vec<u32> = vec![0; map_width * map_height];
    for i in 0..map_height * map_width {
        if i % 6 == 0 {
            map[i] = 0xFFFF00;
        }
    }

    let mut camera_position = 0;

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
    let mut y = 0;
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                camera[i * WIDTH + j] = map[camera_position + j + i * map_width];
            }
        }
        if window.is_key_down(Key::Up) {
            if camera_position as i32 - map_width as i32 >= 0 {
                camera_position -= map_width;
            }
        } 
        if window.is_key_down(Key::Down) {
            if (camera_position + HEIGHT * map_width) <= (map_height * map_width - 1) {
                camera_position += map_width;
            }
        } 
        if window.is_key_down(Key::Right) {
            camera_position += 1;
        } 
        if window.is_key_down(Key::Left) {
            camera_position -= 1;
        } 
        window.update_with_buffer(&camera, WIDTH, HEIGHT).unwrap();
    }
}
