
pub type ReaderCallback = Box<dyn Fn() -> Option<String>>;

pub struct Reader {
    line: String,
    offset: usize,
    position: usize,
    callback: ReaderCallback,
}

impl Reader {
    pub fn new(callback: ReaderCallback) -> Self {
        Self {
            line: String::new(),
            offset: 0,
            position: 0,
            callback,
        }
    }

    fn try_peek(&self) -> Option<char> {
        self.line.chars().nth(self.position - self.offset)
    }

    fn req_peek(&mut self) -> Option<char> {
        match self.line.chars().nth(self.position - self.offset) {
            Some(c) => Some(c),
            None => {
                self.offset = self.position;
                self.line = (self.callback)()?;
                self.try_peek()
            },
        }
    }

    pub fn peek(&mut self, require: bool) -> Option<char> {
        if require { self.req_peek() } else { self.try_peek() }
    }

    pub fn try_next(&mut self) -> Option<char> {
        let result = self.line.chars().nth(self.position - self.offset)?;
        self.position += 1;
        Some(result)
    }

    pub fn req_next(&mut self) -> Option<char> {
        match self.line.chars().nth(self.position - self.offset) {
            Some(c) => {
                self.position += 1;
                Some(c)
            },
            None => {
                self.offset = self.position + 1;
                self.line = (self.callback)()?;
                self.try_next()
            },
        }
    }

    pub fn next(&mut self, require: bool) -> Option<char> {
        if require { self.req_next() } else { self.try_next() }
    }

    pub fn position(&self) -> usize { self.position }
}
