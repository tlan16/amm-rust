use std::thread::sleep;
use std::time;

use log::debug;
use mouse_rs::types::Point;
use mouse_rs::Mouse;

use crate::get_random_int::{get_random_i32, get_random_u64};
use crate::logger::init_logger;

mod get_random_int;
mod logger;

fn main() {
    init_logger();

    let mut options = MainLoop {
        mouse: Mouse::new(),
        previous_x: None,
        previous_y: None,
        idle_counter: 0,
        idle_margin: 100,
        idle_detection_delay_seconds: 1,
        idle_threshold: 4,
    };

    loop {
        main_loop(&mut options);
    }
}

struct MainLoop {
    mouse: Mouse,
    previous_x: Option<i32>,
    previous_y: Option<i32>,
    idle_counter: u64,
    idle_margin: i32,
    idle_detection_delay_seconds: u64,
    idle_threshold: u64,
}

fn main_loop(options: &mut MainLoop) {
    let Point { x, y } = options.mouse.get_position().expect("F_C_P");
    let gap = Point {
        x: (options.previous_x.unwrap_or(x) - x).abs(),
        y: (options.previous_y.unwrap_or(y) - y).abs(),
    };
    options.previous_x = Some(x);
    options.previous_y = Some(y);

    debug!("gap: {:?}", gap);
    debug!("idle counter: {}", options.idle_counter);
    debug!(
        "previous position: ({}, {})",
        options.previous_x.unwrap(),
        options.previous_y.unwrap()
    );

    if gap.x > options.idle_margin || gap.y > options.idle_margin {
        sleep(time::Duration::from_secs(
            options.idle_detection_delay_seconds,
        ));
        options.idle_counter = 0;
        debug!("reset idle counter");
        return;
    }

    if options.idle_counter < options.idle_threshold {
        sleep(time::Duration::from_secs(
            options.idle_detection_delay_seconds,
        ));
        options.idle_counter += 1;
        debug!("increment idle counter to {}", options.idle_counter);
        return;
    }

    debug!("moving mouse");
    options
        .mouse
        .move_to(
            (options
                .previous_x
                .unwrap_or(options.mouse.get_position().unwrap().x))
                + get_random_i32(-10, 10),
            (options
                .previous_y
                .unwrap_or(options.mouse.get_position().unwrap().y))
                + get_random_i32(-10, 10),
        )
        .expect("F_MV");
    sleep(time::Duration::from_secs(get_random_u64(
        options.idle_detection_delay_seconds,
        5u64,
    )));
}
