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

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600 * 2)));
    #[derive(PartialEq)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let mut dir = Direction::Right;
    let mut head_position = 2;
    let mut tail: Vec<usize> = vec![0, 1];
    let mut apple = 400;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::W) && dir != Direction::Down {
            // Controls
            dir = Direction::Up;
        } else if window.is_key_down(Key::S) && dir != Direction::Up {
            dir = Direction::Down;
        } else if window.is_key_down(Key::D) && dir != Direction::Left{
            dir = Direction::Right;
        } else if window.is_key_down(Key::A) && dir != Direction::Right{
            dir = Direction::Left;
        }

        buffer[head_position] = 0; // clear the previous head_position
        tail.push(head_position);
        if head_position != apple {
            buffer[tail[0]] = 0;
            tail.drain(..1);
        } else {
            apple = 800;
        }

        match dir {
            Direction::Up => {
                if (head_position as i32 - WIDTH as i32) < 0 {
                    // if the head_position needs to roll over
                    head_position = WIDTH * HEIGHT - (WIDTH - head_position);
                } else {
                    head_position = head_position - WIDTH;
                }
            }
            Direction::Down => {
                if head_position + WIDTH > WIDTH * HEIGHT {
                    // ''
                    head_position = (head_position + WIDTH) - WIDTH * HEIGHT;
                } else {
                    head_position += WIDTH;
                }
            }
            Direction::Left => {
                if head_position % WIDTH == 0 && head_position != 0 && head_position != WIDTH {
                    // ''
                    head_position = head_position - 1 + WIDTH;
                } else if head_position == 0 {
                    head_position = WIDTH - 1;
                } else {
                    head_position -= 1;
                }
            }
            Direction::Right => {
                if (head_position + 1) % WIDTH == 0 {
                    // ''
                    head_position = head_position + 1 - WIDTH;
                } else {
                    head_position += 1;
                }
            }
        }

        if buffer[head_position] == 0xFFFF00 {  // check if head collided with tail and reset
            head_position = 2;
            for i in tail.iter() {
                buffer[*i] = 0;
            }
            tail = vec![0, 1];
            apple = 400;
        }
        buffer[head_position] = 0xFFFF00;

        for i in tail.iter() {
            buffer[*i] = 0xFFFF00;
        }
        buffer[apple] = 0xFF0000;
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
