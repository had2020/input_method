use std::io::{self, Read, Write};

// This method is very similar to the python way of collecting input.
// The user can type a input and see it on their terimal,
// and once they press enter it is submited, just like the python input method.
pub fn input() -> String {
    let mut collected_text = String::new();

    loop {
        let mut key_buffer = [0u8; 8];
        let bytes_read = io::stdin().read(&mut key_buffer).unwrap();
        let slice = &key_buffer[..bytes_read];

        for byte in slice {
            let pressed_key = match byte {
                10 | 13 => "Enter".to_string(),
                127 => "Backspace".to_string(),
                32..=126 => (*byte as char).to_string(),
                _ => {
                    //eprintln!("unknown byte: {}", byte);
                    "".to_string()
                }
            };

            if pressed_key == "Enter" {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().unwrap();
                return collected_text;
            } else if pressed_key == "Backspace" {
                collected_text.pop();
            } else {
                collected_text.push_str(&pressed_key);
            }

            print!("\r{}", collected_text);
            io::stdout().flush().unwrap();
        }
    }
}
