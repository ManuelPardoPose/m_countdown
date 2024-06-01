use termion::{clear, color::{self}, cursor, style};
use crate::{models::config::Config, util::ascii_utils};

pub struct Counter {
    minutes: i8,
    seconds: i8,

    pos: Vec<i16>,
    vel: Vec<i16>,

    bouncing: bool,
    counting: bool,
    beautify: bool,

    curr_width: i16,
    curr_height: i16,
}

impl Counter {
    pub fn new(minutes: i8, seconds: i8, bouncing: bool, beautify: bool, vel: Vec<i16>) -> Self {
        let instance = Self {
            minutes,
            seconds,
            pos: Vec::from([1, 1]),
            vel,
            bouncing,
            counting: true,
            beautify,
            curr_width: 0,
            curr_height: 0,
        };
        return instance;
    }

    pub fn is_finished(&self) -> bool {
        if self.minutes <= 0 && self.seconds <= 0 {
            return true;
        }
        return false;
    }

    pub fn is_counting(&self) -> bool {
        return self.counting;
    }

    pub fn toggle_counting(&mut self) {
        self.counting = !self.counting;
    }

    pub fn decrement(&mut self) {
        if self.seconds > 0 {
            self.seconds -= 1;
            return;
        }
        if self.minutes > 0 {
            self.minutes -= 1;
            self.seconds = 59;
        }
    }

    pub fn set_pos(&mut self, new_pos: Vec<i16>) {
        if new_pos[0] < 1 || new_pos[1] < 1 {
            return;
        }
        self.pos = new_pos;
    }

    pub fn render(&mut self, width: u16, height: u16, config: &Config) {
        // if timer is supposed to move
        if self.bouncing {
            // move
            self.pos[0] = self.pos[0] + self.vel[0];
            self.pos[1] = self.pos[1] + self.vel[1];

            // bounds check (made it complicated because of goto safety)
            if self.pos[0] > width as i16 - self.curr_width {
                self.vel[0] = -self.vel[0];
                self.pos[0] = width as i16 - self.curr_width;
            }
            if self.pos[0] < 1 {
                self.vel[0] = -self.vel[0];
                self.pos[0] = 1;
            }
            if self.pos[1] > height as i16 - self.curr_height {
                self.vel[1] = -self.vel[1];
                self.pos[1] = height as i16 - self.curr_height;
            }
            if self.pos[1] < 1 {
                self.vel[1] = -self.vel[1];
                self.pos[1] = 1;
            }
        } else {
            self.pos[0] = (width / 2) as i16 - (self.curr_width / 2);
            self.pos[1] = (height / 2) as i16 - (self.curr_height / 2);
        }

        //render
        let x = self.pos[0] as u16;
        let y = self.pos[1] as u16;

        let clear_all = clear::All;
        let bold = style::Bold;
        // let colors = config.get_colors();
        let (col1, col2, col3) = config.get_colors();

        println!("{}", clear_all);

        let minutes_formatted = self.format_minutes();
        for line_num in 0..minutes_formatted.len() {
            println!("{}{}{}{}",
                cursor::Goto(x, y + line_num as u16),
                bold,
                color::Fg(color::Rgb(col1.0, col1.1, col1.2)),
                minutes_formatted[line_num].0
            );
        }

        let separator_formatted = self.format_separator();
        for line_num in 0..separator_formatted.len() {
            println!(
                "{}{}{}{}",
                cursor::Goto(x + minutes_formatted[0].1 as u16, y + line_num as u16),
                bold,
                color::Fg(color::Rgb(col2.0, col2.1, col2.2)),
                separator_formatted[line_num].0
            );
        }

        let seconds_formatted = self.format_seconds();
        for line_num in 0..seconds_formatted.len() {
            println!(
                "{}{}{}{}",
                cursor::Goto(x + minutes_formatted[0].1 as u16 + separator_formatted[0].1 as u16, y + line_num as u16),
                bold,
                color::Fg(color::Rgb(col3.0, col3.1, col3.2)),
                seconds_formatted[line_num].0
            );
        }

        self.curr_width = minutes_formatted[0].1 + separator_formatted[0].1 + seconds_formatted[0].1;
        self.curr_height = minutes_formatted.len() as i16;

        return;
    }

    fn format_minutes(&self) -> Vec<(String, i16)> {
        let mut minutes_vec: Vec<(String, i16)>;
        match self.beautify {
            true => {
                let first_digit = self.minutes / 10;
                let second_digit = self.minutes % 10;
                minutes_vec = ascii_utils::ascii_from_digit(char::from_digit(first_digit as u32, 10).unwrap());
                let second_ascii = ascii_utils::ascii_from_digit(char::from_digit(second_digit as u32, 10).unwrap());
                for line_num in 0..minutes_vec.len() {
                    minutes_vec[line_num].0.push_str(&second_ascii[line_num].0);
                    minutes_vec[line_num].1 += second_ascii[line_num].1
                }
            },
            false => {
                minutes_vec = Vec::new();
                let mut minutes_str: String = String::from("");
                if self.minutes < 10 {
                    minutes_str.push_str("0");
                }
                minutes_str.push_str(&self.minutes.to_string());
                minutes_vec.push((minutes_str, 2))
            }
        }
        return minutes_vec;
    }

    fn format_seconds(&self) -> Vec<(String, i16)> {
        let mut seconds_vec: Vec<(String, i16)>;
        match self.beautify {
            true => {
                let first_digit = self.seconds / 10;
                let second_digit = self.seconds % 10;
                seconds_vec = ascii_utils::ascii_from_digit(char::from_digit(first_digit as u32, 10).unwrap());
                let second_ascii = ascii_utils::ascii_from_digit(char::from_digit(second_digit as u32, 10).unwrap());
                for line_num in 0..seconds_vec.len() {
                    seconds_vec[line_num].0.push_str(&second_ascii[line_num].0);
                    seconds_vec[line_num].1 += second_ascii[line_num].1
                }
            },
            false => {
                seconds_vec = Vec::new();
                let mut seconds_str: String = String::from("");
                if self.seconds < 10 {
                    seconds_str.push_str("0");
                }
                seconds_str.push_str(&self.seconds.to_string());
                seconds_vec.push((seconds_str, 2))
            }
        }
        return seconds_vec;
    }

    fn format_separator(&self) -> Vec<(String, i16)> {
        let mut separator_vec: Vec<(String, i16)>;
        match self.beautify {
            true => {
                separator_vec = ascii_utils::ascii_from_digit(':')
            },
            false => {
                separator_vec = Vec::new();
                separator_vec.push((String::from(":"), 1))
            }
        }
        return separator_vec;
    }
}
