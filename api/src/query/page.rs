#[derive(Clone, Copy, Debug)]
pub struct Page {
    number: u64,
    size: u64,
}

impl Page {
    pub const DEFAULT_SIZE: u64 = 25;

    pub fn new(number: u64, size: u64) -> Page {
        Page { number, size }
    }

    pub fn number(&self) -> u64 {
        self.number
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn offset(&self) -> u64 {
        match self.number() {
            0...1 => 0,
            num @ _ => (num - 1) * self.size(),
        }
    }
}

impl Default for Page {
    fn default() -> Page {
        Page::new(1, Page::DEFAULT_SIZE)
    }
}
