// This Rust program implements a terminal countdown.
// author: Manuel Pardo Pose
// version: 1.0

use std::{time::Duration, thread, string::String};
use clap::Parser;
use m_countdown::models::counter::Counter;

#[derive(Parser)]
struct Args {
    /// The init minutes of the countdown
    #[arg(short, long, default_value_t = 5)]
    min: i8,
    /// The init seconds of the countdown
    #[arg(short, long, default_value_t = 0)]
    sec: i8,
    /// Determines if the counter is supposed to bounce/move
    #[arg(short, long, default_value_t = false)]
    bounce: bool,
    /// Determines if the counter is supposed to be ascii art
    #[arg(short, long, default_value_t = false)]
    ascii_mode: bool,
}

fn main() {
    let args = Args::parse();

    let mut term_size = termion::terminal_size().unwrap();
    let (mut width, mut height) = term_size;

    let fps = 15;
    let mut frame_count: u64 = 0;

    let mut counter = Counter::new(
        args.min, args.sec, args.bounce, args.ascii_mode
    );
    let loop_wait_time = Duration::from_millis(1000 / fps);

    counter.render(width, height);

    while !counter.is_finished() {
        if counter.is_counting() && frame_count%fps == 0 {
            counter.decrement();
        }
        thread::sleep(loop_wait_time);
        counter.render(width, height);
        term_size = termion::terminal_size().unwrap();
        width = term_size.0;
        height = term_size.1;

        frame_count+=1;
    }
    let end_wait_duration = Duration::from_secs(3);
    thread::sleep(end_wait_duration);
}
