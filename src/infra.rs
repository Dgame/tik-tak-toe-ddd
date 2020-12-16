use crate::domain::{Field, FieldFormatter, Playground, PlaygroundDisplay, Reader, Writer};

pub struct BracketFieldFormatter;

impl FieldFormatter for BracketFieldFormatter {
    fn format(&self, field: &Field) -> String {
        if let Some(ref mark) = field.get_mark() {
            format!("[{}]", mark)
        } else {
            String::from("[ ]")
        }
    }
}

pub struct TerminalPlaygroundDisplay<F: FieldFormatter> {
    formatter: F,
}

impl<F: FieldFormatter> TerminalPlaygroundDisplay<F> {
    pub fn new(formatter: F) -> Self {
        Self { formatter }
    }
}

impl<F: FieldFormatter> PlaygroundDisplay for TerminalPlaygroundDisplay<F> {
    fn display(&self, playground: &Playground) {
        for (index, field) in playground.get_fields().iter().enumerate() {
            print!("{}", self.formatter.format(field));
            if (index + 1) % 3 == 0 {
                println!();
            }
        }
    }
}

pub struct TerminalWriter;

impl Writer for TerminalWriter {
    fn writeln(&self, output: &str) {
        println!("{}", output);
    }
}

pub struct TerminalReader;

impl Reader for TerminalReader {
    fn readln(&self) -> String {
        use std::io::{self, BufRead};

        let mut line = String::new();
        let stdin = io::stdin();

        stdin
            .lock()
            .read_line(&mut line)
            .expect("could not read from stdin");

        line
    }
}
