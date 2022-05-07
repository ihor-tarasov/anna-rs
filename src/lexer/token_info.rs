#[derive(Debug, Clone)]
pub struct TokenInfo {
    begin: usize,
    length: usize,
}

impl TokenInfo {
    pub fn new(begin: usize, length: usize) -> Self {
        Self { begin, length }
    }

    pub fn begin(&self) -> usize { self.begin }
    pub fn length(&self) -> usize { self.length }
}
