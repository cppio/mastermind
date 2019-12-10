use std::cmp;

pub const PEGS: u8 = 6;
pub const COLORS: u32 = 10;

pub struct Code(u32);

impl Code {
    pub fn compare(&self, code: &Code) -> (u8, u8) {
        let mut this = self.0;
        let mut that = code.0;

        let mut exact = 0;
        let mut counts = [(0, 0); COLORS as usize];

        for _ in 0..PEGS {
            let i = (this % COLORS) as usize;
            let j = (that % COLORS) as usize;

            counts[i].0 += 1;
            counts[j].1 += 1;

            if i == j {
                exact += 1;
            }

            this /= COLORS;
            that /= COLORS;
        }

        let mut partial = counts.iter().map(|(a, b)| cmp::min(a, b)).sum();
        partial -= exact;

        (exact, partial)
    }
}

impl From<u32> for Code {
    fn from(code: u32) -> Code {
        Code(code)
    }
}

impl From<&Code> for u32 {
    fn from(code: &Code) -> u32 {
        code.0
    }
}
