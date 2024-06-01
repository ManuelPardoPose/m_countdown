use font8x8::{BASIC_FONTS, UnicodeFonts};
use rand::random;

use crate::models::style::CharStyle;

pub fn ascii_from_digit(digit: char, char_style: &CharStyle) -> Vec<(String, i16)> {
    let mut ascii: Vec<(String, i16)> = Vec::new();
    let mut styled_digit: char = match char_style {
        CharStyle::Numbers => digit,
        CharStyle::Solid => '█',
        _ => '█'
    };
    if let Some(glyph) = BASIC_FONTS.get(digit) {
        for x in &glyph {
            let mut curr_line:String = String::new();
            let mut actual_length: i16 = 0;
            for bit in 0..8 {
                if let CharStyle::Random = char_style {
                    styled_digit = (0x20u8 + (random::<f32>() * 96.0) as u8) as char;
                }
                match *x & 1 << bit {
                    0 => curr_line.push_str(" "),
                    _ => curr_line.push(styled_digit),
                }
                actual_length += 1;
            }
            ascii.push((curr_line, actual_length));
        }
    }
    return ascii;
}
