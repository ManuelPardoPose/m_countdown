use crate::models::config::RgbCol;

pub fn parse_rgb(input_str: &String) -> Option<RgbCol> {
    if input_str.len() <= 0 {
        return None;
    }
    let split_input: Vec<&str> = input_str.split(",").into_iter().collect();
    let rgb: Vec<u8> = split_input
        .iter()
        .map(|x| x.parse::<u8>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();
    if rgb.len() != 3 {
        return None;
    }
    Some(RgbCol(
        *rgb.get(0).unwrap(),
        *rgb.get(1).unwrap(),
        *rgb.get(2).unwrap(),
    ))
}

pub fn parse_vel(input_str: &String) -> Option<Vec<i16>> {
    if input_str.len() <= 0 {
        return None;
    }
    let split_input: Vec<&str> = input_str.split(",").into_iter().collect();
    let vel: Vec<i16> = split_input
        .iter()
        .map(|x| x.parse::<i16>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();
    if vel.len() != 2 {
        return None;
    }
    Some(vel)
}
