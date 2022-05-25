extern crate minifb;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const MAP_WIDTH: usize = 320;
const MAP_HEIGHT: usize = 160;
const CAMERA_WIDTH: usize = 320;
const CAMERA_HEIGHT: usize = 160;

struct Sprite {
    position: usize,
    width: usize,
    height: usize,
    texture: Vec<u32>,
    render_priority: usize,
}

impl Sprite {
    pub fn new(
        position: usize,
        width: usize,
        height: usize,
        texture: Vec<u32>,
        render_priority: usize,
    ) -> Sprite {
        Sprite {
            position,
            width,
            height,
            texture,
            render_priority,
        }
    }
}

fn main() {
    let mut camera: Vec<u32> = vec![0; CAMERA_WIDTH * CAMERA_HEIGHT];

    let mut map: Vec<u32> = vec![0; MAP_WIDTH * MAP_HEIGHT];
    let mut camera_position = 0;
    let mut objects: Vec<Sprite> = Vec::new();

    let mut player = Sprite::new(20, 3, 3, vec![0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFF0000, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF, 0xFFFFFF], 1);
    objects.push(player);

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

    let mut window =
        Window::new("background test", CAMERA_WIDTH, CAMERA_HEIGHT, options).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() {
        map = vec![0; MAP_WIDTH * MAP_HEIGHT];
        for sprite in &objects {
            // draw the sprites on the map
            for i in 0..sprite.height {
                for j in 0..sprite.width {
                    map[sprite.position + i * MAP_WIDTH + j] = sprite.texture[i * sprite.width + j];
                }
            }
        }
        for i in 0..CAMERA_HEIGHT {
            // transfer the image from the map to the camera
            for j in 0..CAMERA_WIDTH {
                camera[i * CAMERA_WIDTH + j] = map[camera_position + j + i * MAP_WIDTH];
            }
        }
        // Player movement, including checks to not let the code panic
        for key in window.get_keys() {
            match key {
                Key::W => {
                    if (objects[0].position as i32 - MAP_WIDTH as i32) >= 0 {
                        objects[0].position -= MAP_WIDTH;
                    }
                },
                Key::A => {
                    if objects[0].position % MAP_HEIGHT != 0 {
                        objects[0].position -= 1;
                    }
                },
                Key::S => {
                    if objects[0].position < MAP_HEIGHT * MAP_WIDTH {
                            objects[0].position += MAP_WIDTH;
                    }
                }
                Key::D => {
                    if (objects[0].position + 1) % MAP_HEIGHT != 0 {
                        objects[0].position += 1;
                    }
                }
                _ => (),
            }
        }

        window
            .update_with_buffer(&camera, CAMERA_WIDTH, CAMERA_HEIGHT)
            .unwrap();
    }
}
