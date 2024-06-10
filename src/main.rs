// This Rust program implements a terminal countdown.
// author: Manuel Pardo Pose
// version: 1.0

use clap::Parser;
use m_countdown::models::{
    config::{Config, RgbCol},
    counter::Counter,
    style::CharStyle,
};
use m_countdown::util::arg_parser::{parse_rgb, parse_sec, parse_vel};
use std::{string::String, thread, time::Duration};

#[derive(Parser)]
struct Args {
    /// The init minutes of the countdown
    #[arg(short = 'm', long, default_value_t = 5)]
    min: i8,
    /// The init seconds of the countdown
    #[arg(short = 's', long, default_value_t = 0)]
    sec: i8,
    /// Determines if the counter is supposed to bounce/move
    #[arg(short = 'b', long, default_value_t = false)]
    bounce: bool,
    /// Overwrites the first color (RGB separated by colon)
    #[arg(long, default_value_t = String::new())]
    col1: String,
    /// Overwrites the second color (RGB separated by colon)
    #[arg(long, default_value_t = String::new())]
    col2: String,
    /// Overwrites the third color (RGB separated by colon)
    #[arg(long, default_value_t = String::new())]
    col3: String,
    /// The velocity of the counter
    #[arg(short = 'v', long, default_value_t = String::new())]
    vel: String,
    /// Style of the filling chars (0 -> Solid, 1 -> Numbers, 2 -> Random, 3 -> Small)
    #[arg(short = 'c', long, default_value_t = 0)]
    char_style: u8,
}

fn main() {
    let args = Args::parse();

    let mut term_size = termion::terminal_size().unwrap();
    let (mut width, mut height) = term_size;

    let fps = 15;
    let mut frame_count: u64 = 0;

    let mut counter = Counter::new(
        args.min,
        parse_sec(&args.sec),
        args.bounce,
        parse_vel(&args.vel).unwrap_or(vec![1, 1]),
        match args.char_style {
            0 => CharStyle::Solid,
            1 => CharStyle::Numbers,
            2 => CharStyle::Random,
            3 => CharStyle::Small,
            _ => CharStyle::Solid,
        },
    );

    let col1: RgbCol = match parse_rgb(&args.col1) {
        Some(color) => color,
        None => RgbCol(251, 73, 52),
    };
    let col2: RgbCol = match parse_rgb(&args.col2) {
        Some(color) => color,
        None => RgbCol(184, 187, 38),
    };
    let col3: RgbCol = match parse_rgb(&args.col3) {
        Some(color) => color,
        None => RgbCol(250, 189, 47),
    };

    let config = Config::new(col1, col2, col3);

    let loop_wait_time = Duration::from_millis(1000 / fps);

    counter.render(width, height, &config);

    while !counter.is_finished() {
        if counter.is_counting() && frame_count % fps == 0 {
            counter.decrement();
        }
        thread::sleep(loop_wait_time);
        counter.render(width, height, &config);
        term_size = termion::terminal_size().unwrap();
        width = term_size.0;
        height = term_size.1;

        frame_count += 1;
    }
    let end_wait_duration = Duration::from_secs(3);
    thread::sleep(end_wait_duration);
}
