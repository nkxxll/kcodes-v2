use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "input.txt";

    if let Ok(lines) = read_lines(path) {
        // Print the keycodes in a C array format.
        let keycodes = lines_to_keycodes(lines);
        println!("const uint16_t keycodes[] = {{ {} }};", keycodes.join(", "));
    }

    Ok(())
}

fn lines_to_keycodes(lines: Lines<BufReader<File>>) -> Vec<String> {
    let mut keycodes: Vec<String> = Vec::new();
    let keymap = get_keymap();
    for line in lines {
        for key in line.expect("error reading line").split_whitespace() {
            let keycode = match keymap.get(key) {
                Some(code) => code.to_string(),
                None => {
                    eprintln!("Unknown key: {}", key);
                    continue;
                }
            };
            keycodes.push(keycode);
        }
    }
    keycodes
}

// Function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Function to map keys and modifiers to QMK keycodes
fn get_keymap() -> HashMap<&'static str, &'static str> {
    let mut keymap = HashMap::new();

    // Letters
    keymap.insert("A", "KC_A");
    keymap.insert("B", "KC_B");
    keymap.insert("C", "KC_C");
    keymap.insert("D", "KC_D");
    keymap.insert("E", "KC_E");
    keymap.insert("F", "KC_F");
    keymap.insert("G", "KC_G");
    keymap.insert("H", "KC_H");
    keymap.insert("I", "KC_I");
    keymap.insert("J", "KC_J");
    keymap.insert("K", "KC_K");
    keymap.insert("L", "KC_L");
    keymap.insert("M", "KC_M");
    keymap.insert("N", "KC_N");
    keymap.insert("O", "KC_O");
    keymap.insert("P", "KC_P");
    keymap.insert("Q", "KC_Q");
    keymap.insert("R", "KC_R");
    keymap.insert("S", "KC_S");
    keymap.insert("T", "KC_T");
    keymap.insert("U", "KC_U");
    keymap.insert("V", "KC_V");
    keymap.insert("W", "KC_W");
    keymap.insert("X", "KC_X");
    keymap.insert("Y", "KC_Y");
    keymap.insert("Z", "KC_Z");

    // Numbers
    keymap.insert("1", "KC_1");
    keymap.insert("2", "KC_2");
    keymap.insert("3", "KC_3");
    keymap.insert("4", "KC_4");
    keymap.insert("5", "KC_5");
    keymap.insert("6", "KC_6");
    keymap.insert("7", "KC_7");
    keymap.insert("8", "KC_8");
    keymap.insert("9", "KC_9");
    keymap.insert("0", "KC_0");

    // Special characters
    keymap.insert("!", "KC_EXCLAIM");
    keymap.insert("@", "KC_AT");
    keymap.insert("#", "KC_HASH");
    keymap.insert("$", "KC_DOLLAR");
    keymap.insert("%", "KC_PERCENT");
    keymap.insert("^", "KC_CIRCUMFLEX");
    keymap.insert("&", "KC_AMPERSAND");
    keymap.insert("*", "KC_ASTERISK");
    keymap.insert("(", "KC_LPAREN");
    keymap.insert(")", "KC_RPAREN");
    keymap.insert("-", "KC_MINUS");
    keymap.insert("_", "KC_UNDERSCORE");
    keymap.insert("=", "KC_EQUAL");
    keymap.insert("+", "KC_PLUS");

    // Modifiers
    keymap.insert("SHIFT", "KC_LSFT");
    keymap.insert("CTRL", "KC_LCTL");
    keymap.insert("ALT", "KC_LALT");
    keymap.insert("GUI", "KC_LGUI");

    // Special
    keymap.insert("TODO", "KC_TODO");
    keymap.insert("TRANS", "KC_TRNS");
    keymap.insert("NO", "KC_NO");

    keymap
}

#[cfg(test)]
mod tests {
    use super::*;
}
