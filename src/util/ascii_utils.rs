use font8x8::{BASIC_FONTS, UnicodeFonts};

pub fn ascii_from_digit(input: char) -> Vec<(String, i16)> {
    let mut ascii: Vec<(String, i16)> = Vec::new();
    if let Some(glyph) = BASIC_FONTS.get(input) {
        for x in &glyph {
            let mut curr_line:String = String::new();
            let mut actual_length: i16 = 0;
            for bit in 0..8 {
                match *x & 1 << bit {
                    0 => curr_line.push_str(" "),
                    //_ => curr_line.push_str("█"),
                    _ => curr_line.push(input),
                }
                actual_length += 1;
            }
            ascii.push((curr_line, actual_length));
        }
    }
    return ascii;
}
