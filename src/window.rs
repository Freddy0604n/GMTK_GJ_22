extern crate minifb;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const CAMERA_WIDTH: usize = 320;
const CAMERA_HEIGHT: usize = 160;


pub struct Sprite {
    position: usize,
    width: usize,
    height: usize,
    texture: Vec<u32>,
}

impl Sprite {
    pub fn new(
        position: usize,
        width: usize,
        height: usize,
        texture: Vec<u32>,
    ) -> Sprite {
        Sprite {
            position,
            width,
            height,
            texture,
        }
    }
}

pub fn start(mut world: Vec<Sprite>, mut map: Vec<u32>, map_width: usize, map_height: usize) {
    let mut camera: Vec<u32> = vec![0; CAMERA_WIDTH * CAMERA_HEIGHT];

    let mut camera_position = map_width * 10;

    let mut background: Vec<u32> = Vec::new();
    for i in 0..map.len() {
        background.push(map[i]);
    }

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
        Window::new("Game", CAMERA_WIDTH, CAMERA_HEIGHT, options).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() {
        for pixel in 0..background.len() {
            map[pixel] = background[pixel];
        }
        for sprite in &world {
            // draw the sprites on the map
            for i in 0..sprite.height {
                for j in 0..sprite.width {
                    map[sprite.position + i * map_width + j] = sprite.texture[i * sprite.width + j];
                }
            }
        }
        for i in 0..CAMERA_HEIGHT {
            // transfer the image from the map to the camera
            for j in 0..CAMERA_WIDTH {
                camera[i * CAMERA_WIDTH + j] = map[camera_position + j + i * map_width];
            }
        }
        // Player movement, including checks to not let the code panic
        for key in window.get_keys() {
            match key {
                Key::W => {
                    if (world[0].position as i32 - map_width as i32) >= 0 {
                        world[0].position -= map_width;
                    }
                },
                Key::A => {
                    if world[0].position % map_width != 0 {
                        world[0].position -= 1;
                    }
                },
                Key::S => {
                    if world[0].position + map_width * world[0].height < map_height * map_width {
                            world[0].position += map_width;
                    }
                }
                Key::D => {
                    if (world[0].position + world[0].width) % map_width != 0 {
                        world[0].position += 1;
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