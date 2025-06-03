use std::io::{self, Read};
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
pub fn input() -> &'static str {
    let mut collected_text: String = String::new();
    loop {
        let mut key_buffer = [0u8; 3];

        let bytes_read = io::stdin().read(&mut key_buffer).unwrap();

        let pressed_key: &str = match &key_buffer[..bytes_read] {
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
            [32] => " ",
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
            //_ => "�", // find missing keys
            _ => "", // hide missing keys
        };

        if pressed_key == "Enter" {
            break;
        } else if pressed_key == "Backspace" {
        }
    }
    ""
}
