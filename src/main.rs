use mouse_rs::Mouse;
use rand::Rng;
use std::thread::sleep;
use std::time;


fn compute_is_idle(last_position_x: i32, last_position_y: i32, current_position_x: i32, current_position_y: i32) -> bool {
    let threshold = 10;
    let x_diff = (last_position_x - current_position_x).abs();
    let y_diff = (last_position_y - current_position_y).abs();
    return x_diff < threshold && y_diff < threshold
}

fn main() {
    let debug = false;
    let mouse = Mouse::new();
    let mut idle_counter_seconds = 0;
    let idle_timeout_seconds = 30;
    let mut last_position_x = 0;
    let mut last_position_y = 0;

    loop {
        let current_position = mouse.get_position().expect("F_C_P");
        let is_idle = compute_is_idle(last_position_x, last_position_y, current_position.x, current_position.y);
        if debug {
            println!("is_idle: {}. idle counter {}/{}.", is_idle, idle_counter_seconds, idle_timeout_seconds);
        }

        if idle_counter_seconds >= idle_timeout_seconds {
            if !is_idle {
                idle_counter_seconds = 0;
                continue;
            }
            let mut new_position_x = current_position.x + generate_random_integer(-5, 5);
            if new_position_x < 0 {
                new_position_x = 0;
            }
            let mut new_position_y = current_position.y + generate_random_integer(-5, 5);
            if new_position_y < 0 {
                new_position_y = 0;
            }
            mouse
                .move_to(new_position_x, new_position_y)
                .expect("F_MV");
        } else {
            idle_counter_seconds += 1;
        }

        last_position_x = current_position.x;
        last_position_y = current_position.y;
        sleep(time::Duration::from_secs(1));
    }
}

fn generate_random_integer(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
