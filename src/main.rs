extern crate minifb;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const MAP_WIDTH: usize = 320;
const MAP_HEIGHT: usize = 160;

struct Sprite {
    position: usize,
    width: usize,
    height: usize,
    texture: Vec<u32>,
    render_priority: usize,
}

fn main() {
    let mut camera: Vec<u32> = vec![0; MAP_WIDTH * MAP_HEIGHT];

    let map_width = MAP_WIDTH * 2;
    let map_height = MAP_HEIGHT + 20;
    let mut map: Vec<u32> = vec![0; map_width * map_height];
    let mut camera_position = 0;
    let mut objects: Vec<Sprite> = Vec::new();

    let mut player = Sprite {
        position: 20,
        width: 1,
        height: 1,
        texture: vec![0xFFFFFF],
        render_priority: 1,
    };
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
        Window::new("background test", MAP_WIDTH, MAP_HEIGHT, options).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() {
        for sprite in &objects { // draw the sprites on the map
            for i in 0..sprite.height {
                for j in 0..sprite.width {
                    map[sprite.position + i * map_width + j] = sprite.texture[i* sprite.width + j];
                }
            }
        }       
        for i in 0..MAP_HEIGHT { // transfer the image from the map to the camera
            for j in 0..MAP_WIDTH {
                camera[i * MAP_WIDTH + j] = map[camera_position + j + i * map_width];
            }
        }

        window
            .update_with_buffer(&camera, MAP_WIDTH, MAP_HEIGHT)
            .unwrap();
    }
}
