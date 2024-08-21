use std::io::Stdout;

use terminal::{error, Terminal, Value};

pub fn get_terminal_dimensions(terminal: &Terminal<Stdout>) -> error::Result<(u16, u16)> {
    let size_result = terminal.get(Value::TerminalSize);

    match size_result {
        Ok(retrieved) => match retrieved {
            terminal::Retrieved::TerminalSize(x, y) => return Ok((x, y)),
            terminal::Retrieved::CursorPosition(_, _) => todo!(),
            terminal::Retrieved::Event(_) => todo!(),
        },
        Err(_) => todo!(),
    }
}
