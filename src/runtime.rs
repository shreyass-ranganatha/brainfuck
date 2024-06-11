use std::io::Read;

pub struct Runtime {
    pub p: usize,
    pub array: [u8; 30_000],
}

impl Default for Runtime {
    fn default() -> Self {
        Runtime {
            p: 0,
            array: [0 as u8; 30_000]
        }
    }
}

impl Runtime {
    pub fn shift_right(&mut self) {
        if self.p < 30_000 {
            self.p += 1;
        }
    }

    pub fn shift_left(&mut self) {
        if self.p > 0 {
            self.p -= 1;
        }
    }

    pub fn plus(&mut self) {
        self.array[self.p] += 1;
    }

    pub fn minus(&mut self) {
        self.array[self.p] -= 1;
    }

    pub fn print(&self) {
        print!("{}", self.array[self.p] as char);
    }

    pub fn read(&mut self) {
        let mut bf: [u8; 1] = [0; 1];

        std::io::stdin()
            .read(&mut bf)
            .expect("Couldn't read user input");

        self.array[self.p] = bf[0];
    }

    pub fn value(&self) -> u8 {
        return self.array[self.p];
    }
}
