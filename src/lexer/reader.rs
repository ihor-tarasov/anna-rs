pub struct Reader<'a> {
    data: &'a[u8],
    position: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a[u8]) -> Self {
        Self {
            data,
            position: 0,
        }
    }

    pub fn peek(&self) -> Option<u8> {
        Some(*(self.data.get(self.position)?))
    }

    pub fn next(&mut self) -> Option<u8> {
        let result = *(self.data.get(self.position)?);
        self.position += 1;
        Some(result)
    }

    pub fn position(&self) -> usize { self.position }
}
