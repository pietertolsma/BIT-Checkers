use std::io;

pub const LETTER_MAP: &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

/// Return a letter to int representation (A = 0, B = 1 etc)
pub fn letter_to_digit(letter : char) -> i32 {
    for _i in 0..8 {
        if LETTER_MAP[_i] == letter {
            return _i as i32;
        }
    }
    return -1;
}

/// Convert a valid coordinate string to a usable coordinate.
/// E.g. A0 = (0,0), C3 = (2, 3)
pub fn coords_from_string(input : String) -> (i32, i32) {
    let mut trimmed = input.trim().to_string();
    trimmed.make_ascii_uppercase();
    let x1 = trimmed[0..1].to_string().parse::<char>()
            .expect("Failed to convert letter to char");

    let y1 = trimmed[1..2].to_string().parse::<char>()
            .expect("Failed to convert y string to char");
    let y1 = y1.to_digit(10)
            .expect("Failed to convert y char to digit");

    let coords = (letter_to_digit(x1) as i32, y1 as i32);
    return coords;
}

/// Prompt user to press enter.
pub fn prompt_keypress() {
    println!("Press enter to continue...");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Did not enter a correct string");
}
