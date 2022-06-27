use std::io::{self, BufRead, Write};
use std::process::{self};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut stdin_handle = io::stdin().lock();
    let mut stdout_handle = io::stdout().lock();

    loop {
        stdout_handle.write_all(b"db > ")?;
        stdout_handle.flush()?;

        let bytes_read = stdin_handle.read_line(&mut input)?;
        if bytes_read == 0 {
            stdout_handle.write_all(b"\nError reading input\n")?;
            process::exit(1);
        }
        {
            // Ignore trailing newline
            let input = &input[..input.len() - 1];
            if input.starts_with('.') {
                // Meta commands for inspection.
                match input {
                    ".exit" => break,
                    _ => {
                        stdout_handle.write_all(b"Unrecognized command: '")?;
                        stdout_handle.write_all(input.as_bytes())?;
                        stdout_handle.write_all(b"'\n")?;
                        stdout_handle.flush()?;
                    }
                };
            } else {
                // SQL commands for inspection.
                match input {
                    _ => {
                        stdout_handle.write_all(b"Unrecognized query: '")?;
                        stdout_handle.write_all(input.as_bytes())?;
                        stdout_handle.write_all(b"'\n")?;
                        stdout_handle.flush()?;
                    }
                }
            }
        }
        input.clear();
    }
    Ok(())
}
