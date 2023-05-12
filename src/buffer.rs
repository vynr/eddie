//make a struct buffer
//code in struct environment uses different syntax! remember

use std::fs::File;
use std::io::{stdin, BufRead, BufWriter, Write};
use std::ops::Range;

// https://catonmat.net/ftp/ed.unix.text.editor.cheat.sheet.txt
#[derive(Debug, Clone, Default)]
pub struct Buffer {
    // Lines (vector of string type)
    pub name: Option<String>,
    pub saved: bool,
    pub lines: Vec<String>,
    // Pointer to what line of the buffer we're "on" at the current moment
    pub current_line: usize,
}

//buffer operations: insert, remove, switch which line you're on
impl Buffer {
    /// Takes a mutable reference to the buffer (since it needs to change the
    /// Buffer), and an optional line to insert text at (if none given it goes
    /// With the current one)
    fn insert_at_index(&mut self, line: usize) {
        self.current_line = line;

        for text in stdin()
            .lock()
            .lines()
            .map(|r| r.expect("error reading stdin"))
        {
            if text == "." {
                break;
            }
            self.lines.insert(self.current_line, text);
            self.current_line += 1;
            self.saved = false;
        }
    }

    // checks range given is valid, then inserts text before current line
    pub fn insert(&mut self, line: usize) -> Result<(), &'static str> {
        Ok(self.insert_at_index(line.max(1) - 1))
    }
    // calls insert, but tacks line on after the current line instead of before
    pub fn append(&mut self, line: usize) -> Result<(), &'static str> {
        Ok(self.insert_at_index(line))
    }

    pub fn write(&mut self, name: Option<String>) -> Result<(), &'static str> {
        let name = name
            .or_else(|| self.name.clone())
            .ok_or("No current filename")?;

        self.name = Some(name.clone());

        let file = File::create(name).map_err(|_| "Unable to open file for writing")?;

        let mut total = 0;
        let mut writer = BufWriter::new(file);
        for line in self.lines.iter() {
            total += line.len() + "\n".len();
            writeln!(writer, "{}", line).map_err(|_| "I/O error")?;
        }

        println!("{}", total);
        self.saved = true;

        Ok(())
    }
}
