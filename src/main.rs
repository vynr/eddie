mod buffer;

use buffer::Buffer;
use regex::Regex;
use std::io;
use std::io::Write;

fn main() {
    let command_re: Regex = Regex::new(r"^(\d+)?\s*([a-zA-Z])(?:\s+(.*))?$").unwrap();

    let mut error_msg = None;

    let mut buffer = Buffer::default();

    // let mut error = None;
    loop {
        print!("*");
        let _ = io::stdout().flush();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("error reading stdin");

        let captures = match command_re.captures(input.trim_end()) {
            Some(captures) => captures,
            None => {
                error_msg = Some("Unable to parse command");
                eprintln!("?");
                continue;
            }
        };

        let line = captures
            .get(1)
            .map(|s| s.as_str().parse().unwrap())
            .unwrap_or(buffer.current_line);

        if line > buffer.lines.len() {
            error_msg = Some("Invalid address");
            eprintln!("?");
            continue;
        }

        let command = captures.get(2).map(|s| s.as_str()).unwrap();

        let argument = captures.get(3).map(|s| s.as_str().to_string());

        let result = match command {
            "i" => buffer.insert(line),
            "a" => buffer.append(line),
            "w" => buffer.write(argument),
            "p" => {
                for line in buffer.lines.iter() {
                    println!("{line}");
                }
                Ok(())
            }
            "h" => {
                if let Some(msg) = error_msg.clone() {
                    eprintln!("{msg}");
                }
                Ok(())
            }
            "n" => {
                for (i, line) in buffer.lines.iter().enumerate() {
                    println!("{}\t{line}", i + 1);
                }
                Ok(())
            }
            "q" => {
                if buffer.saved {
                    break;
                } else {
                    Err("Warning: buffer modified")
                }
            }
            "Q" => break,
            _ => Err("Unrecognized command"),
        };

        if let Err(msg) = result {
            error_msg = Some(msg);
            eprintln!("?");
        }
    }
}
