// This Rust program implements a terminal countdown.
// author: Manuel Pardo Pose
// version: 1.0

use std::{time::Duration, thread, string::String};
use clap::Parser;
use m_countdown::models::{config::{Config, RgbCol}, counter::Counter, style::CharStyle};

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
    /// Determines if the counter is supposed to be ascii art
    #[arg(short = 'a', long, default_value_t = false)]
    ascii_mode: bool,
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
    /// Style of the filling chars (0 -> Solid, 1 -> Numbers, 2 -> Random)
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
        args.sec,
        args.bounce,
        args.ascii_mode,
        parse_vel(&args.vel).unwrap_or(vec![1,1]),
        match args.char_style {
            0 => CharStyle::Solid,
            1 => CharStyle::Numbers,
            2 => CharStyle::Random,
            _ => CharStyle::Numbers
        }
    );

    let col1: RgbCol = match parse_rgb(&args.col1) {
        Some(color) => color,
        None => RgbCol(251, 73, 52)
    };
    let col2: RgbCol = match parse_rgb(&args.col2) {
        Some(color) => color,
        None => RgbCol(184, 187, 38)
    };
    let col3: RgbCol = match parse_rgb(&args.col3) {
        Some(color) => color,
        None => RgbCol(250, 189, 47)
    };

    let config = Config::new(col1, col2, col3);

    let loop_wait_time = Duration::from_millis(1000 / fps);

    counter.render(width, height, &config);

    while !counter.is_finished() {
        if counter.is_counting() && frame_count%fps == 0 {
            counter.decrement();
        }
        thread::sleep(loop_wait_time);
        counter.render(width, height, &config);
        term_size = termion::terminal_size().unwrap();
        width = term_size.0;
        height = term_size.1;

        frame_count+=1;
    }
    let end_wait_duration = Duration::from_secs(3);
    thread::sleep(end_wait_duration);
}

fn parse_rgb(input_str: &String) -> Option<RgbCol> {
    if input_str.len() <= 0 {
        return None;
    }
    let split_input: Vec<&str> = input_str.split(",").into_iter().collect();
    let rgb: Vec<u8> = split_input.iter()
        .map(|x| x.parse::<u8>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();
    if rgb.len() != 3 {
        return None
    }
    Some(RgbCol(*rgb.get(0).unwrap(), *rgb.get(1).unwrap(), *rgb.get(2).unwrap()))
}

fn parse_vel(input_str: &String) -> Option<Vec<i16>> {
    if input_str.len() <= 0 {
        return None;
    }
    let split_input: Vec<&str> = input_str.split(",").into_iter().collect();
    let vel: Vec<i16> = split_input.iter()
        .map(|x| x.parse::<i16>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();
    if vel.len() != 2 {
        return None
    }
    Some(vel)
}
