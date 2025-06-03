use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time::Duration;

/// Will halt the program till a key is pressed, if a key is pressed the &str representation is returned.
/// Note: the escape key requires 3 presses due to a shortcut that lets this code be lightweight.
pub fn key_pressed() -> &'static str {
    let mut key_buf = [0u8; 3];
    let mut total_read = 0;
    let stdin = io::stdin();

    let read_now = stdin.lock().read(&mut key_buf[total_read..]).unwrap();
    total_read += read_now;

    if key_buf[..total_read] == [27] {
        for _ in 0..2 {
            sleep(Duration::from_millis(30));
            if let Ok(n) = stdin.lock().read(&mut key_buf[total_read..]) {
                if n == 0 {
                    break;
                }
                total_read += n;
            } else {
                break;
            }
        }
    }

    let pressed_key: &str = match &key_buf[..total_read] {
        // escape sequences
        [27, 27, 27] => "Esc", // Note: you have to press esc threee times, due to design.

        // function keys
        [27, 79, 80] => "F1",
        [27, 79, 81] => "F2",
        [27, 79, 82] => "F3",
        [27, 79, 83] => "F4",

        // arrow keys
        [27, 91, 65] => "Up",
        [27, 91, 66] => "Down",
        [27, 91, 67] => "Right",
        [27, 91, 68] => "Left",

        // lowercase letter keys
        [97] => "a",
        [98] => "b",
        [99] => "C",
        [100] => "d",
        [101] => "e",
        [102] => "f",
        [103] => "g",
        [104] => "h",
        [105] => "i",
        [106] => "j",
        [107] => "k",
        [108] => "l",
        [109] => "m",
        [110] => "n",
        [111] => "o",
        [112] => "p",
        [113] => "q",
        [114] => "r",
        [115] => "s",
        [116] => "t",
        [117] => "u",
        [118] => "v",
        [119] => "w",
        [120] => "x",
        [121] => "y",
        [122] => "z",

        // uppercase letter keys
        [65] => "A",
        [66] => "B",
        [67] => "C",
        [68] => "D",
        [69] => "E",
        [70] => "F",
        [71] => "G",
        [72] => "H",
        [73] => "I",
        [74] => "J",
        [75] => "K",
        [76] => "L",
        [77] => "M",
        [78] => "N",
        [79] => "O",
        [80] => "P",
        [81] => "Q",
        [82] => "R",
        [83] => "S",
        [84] => "T",
        [85] => "U",
        [86] => "V",
        [87] => "W",
        [88] => "X",
        [89] => "Y",
        [90] => "Z",

        // numbers
        [48] => "0",
        [49] => "1",
        [50] => "2",
        [51] => "3",
        [52] => "4",
        [53] => "5",
        [54] => "6",
        [55] => "7",
        [56] => "8",
        [57] => "9",

        // special characters
        [32] => "Space",
        [9] => "Tab",
        [10] => "Enter",
        [13] => "Enter",
        [127] => "Backspace",
        [33] => "!",
        [34] => "\"",
        [35] => "#",
        [36] => "$",
        [37] => "%",
        [38] => "&",
        [39] => "'",
        [40] => "(",
        [41] => ")",
        [42] => "*",
        [43] => "+",
        [44] => ",",
        [45] => "-",
        [46] => ".",
        [47] => "/",
        [58] => ":",
        [59] => ";",
        [60] => "<",
        [61] => "=",
        [62] => ">",
        [63] => "?",
        [64] => "@",
        [91] => "[",
        [92] => "\\",
        [93] => "]",
        [94] => "^",
        [95] => "_",
        [96] => "`",
        [123] => "{",
        [124] => "|",
        [125] => "}",
        [126] => "~",

        // fail case
        _ => "unknown",
    };
    pressed_key
}

// This method is very similar to the python way of collecting input.
// The user can type a input and see it on their terimal,
// and once they press enter it is submited, just like th python input method.
pub fn input() -> String {
    let mut collected_text: String = String::new();

    loop {
        let mut key_buffer = [0u8; 3];

        let bytes_read = io::stdin().read(&mut key_buffer).unwrap();

        let pressed_key: String = match &key_buffer[..bytes_read] {
            // lowercase letter keys
            [97] => "a".to_string(),
            [98] => "b".to_string(),
            [99] => "C".to_string(),
            [100] => "d".to_string(),
            [101] => "e".to_string(),
            [102] => "f".to_string(),
            [103] => "g".to_string(),
            [104] => "h".to_string(),
            [105] => "i".to_string(),
            [106] => "j".to_string(),
            [107] => "k".to_string(),
            [108] => "l".to_string(),
            [109] => "m".to_string(),
            [110] => "n".to_string(),
            [111] => "o".to_string(),
            [112] => "p".to_string(),
            [113] => "q".to_string(),
            [114] => "r".to_string(),
            [115] => "s".to_string(),
            [116] => "t".to_string(),
            [117] => "u".to_string(),
            [118] => "v".to_string(),
            [119] => "w".to_string(),
            [120] => "x".to_string(),
            [121] => "y".to_string(),
            [122] => "z".to_string(),

            // uppercase letter keys
            [65] => "A".to_string(),
            [66] => "B".to_string(),
            [67] => "C".to_string(),
            [68] => "D".to_string(),
            [69] => "E".to_string(),
            [70] => "F".to_string(),
            [71] => "G".to_string(),
            [72] => "H".to_string(),
            [73] => "I".to_string(),
            [74] => "J".to_string(),
            [75] => "K".to_string(),
            [76] => "L".to_string(),
            [77] => "M".to_string(),
            [78] => "N".to_string(),
            [79] => "O".to_string(),
            [80] => "P".to_string(),
            [81] => "Q".to_string(),
            [82] => "R".to_string(),
            [83] => "S".to_string(),
            [84] => "T".to_string(),
            [85] => "U".to_string(),
            [86] => "V".to_string(),
            [87] => "W".to_string(),
            [88] => "X".to_string(),
            [89] => "Y".to_string(),
            [90] => "Z".to_string(),

            // numbers
            [48] => "0".to_string(),
            [49] => "1".to_string(),
            [50] => "2".to_string(),
            [51] => "3".to_string(),
            [52] => "4".to_string(),
            [53] => "5".to_string(),
            [54] => "6".to_string(),
            [55] => "7".to_string(),
            [56] => "8".to_string(),
            [57] => "9".to_string(),

            // special characters
            [32] => " ".to_string(),
            [10] => "Enter".to_string(),
            [13] => "Enter".to_string(),
            [127] => "Backspace".to_string(),
            [33] => "!".to_string(),
            [34] => "\"".to_string(),
            [35] => "#".to_string(),
            [36] => "$".to_string(),
            [37] => "%".to_string(),
            [38] => "&".to_string(),
            [39] => "'".to_string(),
            [40] => "(".to_string(),
            [41] => ")".to_string(),
            [42] => "*".to_string(),
            [43] => "+".to_string(),
            [44] => ",".to_string(),
            [45] => "-".to_string(),
            [46] => ".".to_string(),
            [47] => "/".to_string(),
            [58] => ":".to_string(),
            [59] => ";".to_string(),
            [60] => "<".to_string(),
            [61] => "=".to_string(),
            [62] => ">".to_string(),
            [63] => "?".to_string(),
            [64] => "@".to_string(),
            [91] => "[".to_string(),
            [92] => "\\".to_string(),
            [93] => "]".to_string(),
            [94] => "^".to_string(),
            [95] => "_".to_string(),
            [96] => "`".to_string(),
            [123] => "{".to_string(),
            [124] => "|".to_string(),
            [125] => "}".to_string(),
            [126] => "~".to_string(),

            // fail case
            //_ => "�".to_string(), // find missing keys
            _ => "".to_string(), // hide missing keys
        };

        if pressed_key == "Enter" {
            break;
        } else if pressed_key == "Backspace" {
            collected_text.pop();
        } else {
            collected_text.push_str(&pressed_key);
            print!("\r{}", collected_text);
            io::stdout().flush().unwrap();
        }
    }
    collected_text
}
