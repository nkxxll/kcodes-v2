use std::ascii::AsciiExt;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    let path = "input.txt";

    if let Ok(input) = read_lines(path) {
        // Print the keycodes in a C array format.
        let keycodes = lines_to_keycodes(input);
        println!("const uint16_t keycodes[] = {{ {} }};", keycodes.join(", "));
    }

    Ok(())
}

fn lines_to_keycodes(input: String) -> Vec<String> {
    let mut keycodes: Vec<String> = Vec::new();
    let keymap = get_keymap();
    for key in input.split_whitespace() {
        let upper: &str = &key.to_ascii_uppercase();
        let keycode = match keymap.get(upper) {
            Some(code) => code.to_string(),
            None => {
                eprintln!("Unknown key: {}", key);
                continue;
            }
        };
        keycodes.push(keycode);
    }
    keycodes
}

// Function to read lines from a file
fn read_lines<P>(filename: P) -> Result<String>
where
    P: AsRef<Path>,
{
    read_to_string(filename)
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
    use insta::assert_snapshot;

    fn vec_to_string(vec: Vec<String>) -> String {
        vec.iter().fold("vec: ".to_string(), |a, b| a + b + "; ")
    }

    #[test]
    fn kcodes_test1() {
        let input: String = "a b c".to_string();
        let keycodes: Vec<String> = lines_to_keycodes(input);
        assert_snapshot!("snap_test_1", vec_to_string(keycodes))
    }

    #[test]
    fn kcodes_test2() {
        let input: String = "a\nb\nc".to_string();
        let keycodes: Vec<String> = lines_to_keycodes(input);
        assert_snapshot!("snap_test_2", vec_to_string(keycodes))
    }

    #[test]
    fn kcodes_test3() {
        let input: String = "a\n\tb\n\t\tc".to_string();
        let keycodes: Vec<String> = lines_to_keycodes(input);
        assert_snapshot!("snap_test_3", vec_to_string(keycodes))
    }

    #[test]
    fn kcodes_test4() {
        let input: String = "SHIFT a b c GUI NO".to_string();
        let keycodes: Vec<String> = lines_to_keycodes(input);
        assert_snapshot!("snap_test_4", vec_to_string(keycodes))
    }
}
