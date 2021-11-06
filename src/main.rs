use macroquad::{prelude::*, rand::srand};

const BOARD_WIDTH: i16 = 6;
const BOARD_HEIGHT: i16 = 16;

mod board;
use board::Board;

#[macroquad::main("Columns")]
async fn main() {
    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 320.0, 320.0));
    let mut board = Board::new(BOARD_WIDTH, BOARD_HEIGHT).await;
    // let mut dt = 0.0;
    let mut dt_input = 0.0;

    let input_update_rate = 0.0;
    let debounce_time = 0.15;

    let mut left = false;
    let mut right = false;
    let mut up = false;
    let mut down = false;

    let mut left_press_time = 0.0;
    let mut right_press_time = 0.0;
    let mut up_press_time = 0.0;
    let mut down_press_time = 0.0;

    srand(macroquad::miniquad::date::now() as u64);

    loop {
        // dt += get_frame_time();
        dt_input += get_frame_time();

        left_press_time += get_frame_time();
        right_press_time += get_frame_time();
        up_press_time += get_frame_time();
        down_press_time += get_frame_time();

        clear_background(GRAY);

        if left_press_time >= debounce_time {
            if is_key_down(KeyCode::Left) {
                left = true;
                left_press_time = 0.0;
            }
        }
        if right_press_time >= debounce_time {
            if is_key_down(KeyCode::Right) {
                right = true;
                right_press_time = 0.0;
            }
        }
        if up_press_time >= debounce_time {
            if is_key_down(KeyCode::Up) {
                up = true;
                up_press_time = 0.0;
            }
        }
        if down_press_time >= debounce_time {
            if is_key_down(KeyCode::Down) {
                down = true;
                down_press_time = 0.0;
            }
        }

        if dt_input >= input_update_rate {
            dt_input -= input_update_rate;
            board.handle_input(left, right, up, down);
            left = false;
            right = false;
            up = false;
            down = false;
        }

        board.update(get_frame_time());
        board.render();

        set_camera(&camera);
        next_frame().await
    }
}
