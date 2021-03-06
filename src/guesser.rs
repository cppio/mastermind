use super::code::{Code, COLORS, PEGS};

use std::mem;

pub struct CodeGuesser(Code, Vec<(Code, (u8, u8))>);

impl CodeGuesser {
    pub fn new() -> CodeGuesser {
        CodeGuesser(0.into(), Vec::new())
    }

    pub fn guess(&self) -> &Code {
        &self.0
    }

    fn consistent(&self, code: &Code) -> bool {
        self.1
            .iter()
            .all(|(prev_code, prev_reply)| prev_code.compare(code) == *prev_reply)
    }

    pub fn filter(&mut self, reply: (u8, u8)) -> Option<()> {
        let guess = (u32::from(&self.0)..COLORS.pow(PEGS as u32))
            .map(Code::from)
            .find(|code| self.0.compare(code) == reply && self.consistent(code))?;
        self.1.push((mem::replace(&mut self.0, guess), reply));
        Some(())
    }
}
