use std::io::{self, Write};

///todo: Does not work. fix
pub fn clean_monitor() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}
