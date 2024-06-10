use font8x8::{UnicodeFonts, BASIC_FONTS};
use rand::random;

use crate::models::style::CharStyle;

pub fn ascii_from_digit(digit: char, char_style: &CharStyle) -> Vec<(String, i16)> {
    let mut ascii: Vec<(String, i16)> = Vec::new();
    let mut styled_digit: char = match char_style {
        CharStyle::Numbers => digit,
        CharStyle::Solid => '█',
        CharStyle::Small => digit,
        _ => '█',
    };
    if let CharStyle::Small = char_style {
        ascii.push((String::from(styled_digit), 1));
        return ascii;
    }
    if let Some(glyph) = BASIC_FONTS.get(digit) {
        for x in &glyph {
            let mut curr_line: String = String::new();
            let mut actual_length: i16 = 0;
            for bit in 0..8 {
                if let CharStyle::Random = char_style {
                    styled_digit = random_char_from_charpool();
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

fn random_char_from_charpool() -> char {
    let index = (random::<f32>() * 190.0) as u8;
    if index <= 94 {
        return (0x20u8 + index) as char;
    } else if index <= 189 {
        return (0xA1u8 + (index - 95)) as char;
    }
    return 0x20u8 as char;
}
